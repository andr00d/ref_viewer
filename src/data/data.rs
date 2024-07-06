use std::collections::HashMap;
use std::path::Path;
use core::cmp::Ordering;
use serde_json::Value;
use serde_json::json;

use crate::data::{Data, Folder, Image};
use crate::data::exiftool::Exiftool;
use crate::data::image::Index;

impl Data 
{
    pub fn new(paths: Vec<String>) -> Data 
    {
        let mut exif = Exiftool::new();
        let mut data = Vec::<Folder>::new();
        let mut taglist = HashMap::<String, Vec<Index>>::new();
        
        if exif.is_none()
        {
            return Data {folders:data, exif:exif, taglist:taglist};
        }
        
        for (i, path) in paths.iter().enumerate()
        {
            let result = Self::get_folder_data(&path, i, &mut exif.as_mut().unwrap(), &mut taglist);
            match result
            {
                Ok(x) => data.push(x),
                Err(x) => println!("{}", x),
            };
        }

        return Data {folders:data, exif:exif, taglist:taglist};
    }

    ///////////////////
    // data building //
    ///////////////////

    pub fn exif_available(&self) -> bool
    {
        return self.exif.is_some();
    }

    fn read_json(input: &String) -> Vec<String>
    {
        let vec_empty = Vec::<Value>::new();

        let json = serde_json::from_str::<Value>(input).unwrap_or(json!(""));
        let json = json.as_array().unwrap_or(&vec_empty);
        let mut vector = Vec::<String>::new();

        for value in json
        {
            if value.is_string() {vector.push(value.as_str().unwrap().to_string());}
        }

        return vector;
    }

    fn construct_image(info : &serde_json::value::Value) -> Result<Image, &str>
    {
        let empty = &json!("");
        let file = info.get("SourceFile").unwrap_or(empty).as_str().unwrap_or("");
        if file.len() == 0 {return Err("filename incorrect")};
        
        let str_artists = info.get("Artist").unwrap_or(empty).as_str().unwrap_or("");
        let str_links = info.get("PageName").unwrap_or(empty).as_str().unwrap_or("");
        let str_tags = info.get("UserComment").unwrap_or(empty).as_str().unwrap_or("");
        let size = info.get("ImageSize").unwrap_or(empty).as_str().unwrap_or("");
        
        let artists = Self::read_json(&str_artists.to_string());
        let links = Self::read_json(&str_links.to_string());
        let tags = Self::read_json(&str_tags.to_string());

        let image = Image::new(
            file.to_string(), 
            artists, 
            size.to_string(), 
            links, 
            tags,
        );

        return Ok(image)
    }

    // TODO: test non-existant folder
    fn get_folder_data(path: &String, index: usize, exif: &mut Exiftool, taglist: &mut HashMap::<String, Vec<Index>>) ->  Result<Folder, String>
    {
        let mut btn_path = path.to_string().clone();
        if btn_path.len() > 20
        {
            btn_path = 
            {
                // TODO: split cleanly to nearest folder
                let index = btn_path.char_indices().nth_back(17).unwrap().0;
                ("...".to_owned() + &btn_path[index..]).to_string()
            };
        }

        let mut folder = Folder{
            path: path.to_string(), 
            btn_path: btn_path, 
            collapsed: false,
            images: Vec::new()};

        let output = match exif.get_folder_data(path)
        {
            Ok(x) => x,
            Err(_x) => return Err("Error with exiftool".to_string()),
        };

        if output.len() == 0 {return Ok(folder)}

        let json = match serde_json::from_str::<Value>(&output)
        {
            Ok(x) => x,
            Err(_x) => return Err("Error with json output".to_string()),
        };

        // TODO: handle in type->alphabetical order
        let mut img_index = 0;
        for value in json.as_array().unwrap()
        {
            match Self::construct_image(value)
            {
                Ok(x) => 
                {
                    let index = Index{folder:index, image:img_index};
                    Self::update_tags(&x.artists, &index, taglist);
                    Self::update_tags(&x.tags, &index, taglist);
                    folder.images.push(x);
                    img_index += 1
                }
                Err(_x) => println!("error with image"),
            };
        }

        return Ok(folder);
    } 

    pub fn build_vector(&self, tags: Vec<String>, itags: Vec<String>) -> Vec<Vec<Index>>
    {
        let mut imglist = Vec::new();
        for _ in 0..self.folders.len() {imglist.push(Vec::<Index>::new());}

        match tags.len()
        {
            0 => 
            {
                for (f, folder) in self.folders.iter().enumerate()
                {
                    for i in 0..folder.images.len()
                    {
                        imglist[f].push(Index{folder:f, image:i});
                    }
                } 
            }
            _ =>
            {
                let mut alltags = Vec::new();

                for tag in &tags
                {
                    match self.taglist.get(tag)
                    {
                        Some(vec) => {alltags.append(&mut vec.clone());},
                        None => (),
                    }
                }

                let mut tagcounts = HashMap::new();
                for item in alltags {*tagcounts.entry(item).or_insert(0) += 1;}
                for (tag, count) in tagcounts {if count == tags.len() {imglist[tag.folder].push(tag);}}

                for folder in &mut imglist
                {
                    folder.sort_by(|a, b| {
                        if a.folder < b.folder {Ordering::Less} 
                        else if a.folder > b.folder {Ordering::Greater} 
                        else 
                        {
                            if a.image < b.image {Ordering::Less} 
                            else if a.image > b.image {Ordering::Greater} 
                            else {Ordering::Equal}                    
                        }
                    });
                }
            }
        }

        
        for tag in itags
        {
            match self.taglist.get(&tag)
            {
                Some(vec) => 
                {
                    for index in vec
                    {
                        for folder in &mut imglist
                        {
                            match folder.iter().position(|x| x == index)
                            {
                                Some(index) => {folder.remove(index);},
                                None => {},
                            };
                        }
                    }
                },
                None => (),
            }
        }

        return imglist;
    }

    pub fn get_string_index(&self, path: String) -> Option<Index>
    {
        let img_folder = Path::new(&path).parent().unwrap().to_string_lossy().into_owned();
        for (f, folder) in self.folders.iter().enumerate()
        {
            if folder.path != img_folder {continue;}

            for (i, image) in folder.images.iter().enumerate()
            {
                if image.file == path {return Some(Index{folder:f, image:i});}
            }
        }
        return None;
    }

    //////////////
    // taglist //
    /////////////

    fn update_tags(tags: &Vec<String>, index: &Index, taglist: &mut HashMap::<String, Vec<Index>>)
    {
        for tag in tags
        {
            taglist.entry(tag.clone()).or_default().push(index.clone());
        }
    }

    fn add_taglist(taglist: &mut HashMap::<String, Vec<Index>>, img_index: &Index, tag: &String) -> ()
    {
        match taglist.get_mut(tag)
        {
            Some(vector) =>
            {
                match vector.iter().position(|x| x == img_index)
                {
                    Some(_) => (),
                    None =>
                    {
                        // TODO: is sorting necessary?
                        vector.push(img_index.clone());
                    },
                }
            },
            None => 
            {
                taglist.entry(tag.clone()).or_default().push(img_index.clone());
            },
        };
    }

    fn rem_taglist(taglist: &mut HashMap::<String, Vec<Index>>, img_index: &Index, tag: &String) -> ()
    {
        match taglist.get_mut(tag)
        {
            Some(vector) =>
            {
                match vector.iter().position(|x| x == img_index)
                {
                    Some(index) =>
                    {
                        vector.remove(index);
                        if vector.len() == 0 {taglist.remove(tag);}
                    },
                    None => (),
                }
            },
            None => (),
        };
    }
    
    ///////////////////
    // changing tags //
    ///////////////////

    pub fn get_nr_imgs(&self) -> usize
    {
        let mut count = 0;

        for folder in &self.folders
        {
            count += folder.images.len();
        }

        return count;
    }

    fn build_string(vector: &Vec<String>) -> String
    {
        let mut result = "[".to_string();

        for item in vector
        {
            result.push_str(format!("\"{item}\",").as_str());
        }
        
        if vector.len() > 0 {result.pop();}
        result.push_str("]");
        return result;
    }

    pub fn del_tag(&mut self, img_index: &Index, tag: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];

        img.remove_tag(tag);
        Self::rem_taglist(&mut self.taglist, img_index, tag);

        let output = Self::build_string(&img.tags);
        let _ = self.exif.as_mut().unwrap().set_usercomment(&img.file, &output);
    }

    pub fn add_tag(&mut self, img_index: &Index, tag: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        
        img.add_tag(tag);
        Self::add_taglist(&mut self.taglist, img_index, tag);

        let output = Self::build_string(&img.tags);
        let _ = self.exif.as_mut().unwrap().set_usercomment(&img.file, &output);
    }

    pub fn del_link(&mut self, img_index: &Index, link: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.remove_link(link);
        let output = Self::build_string(&img.links);
        let _ = self.exif.as_mut().unwrap().set_link(&img.file, &output);
    }

    pub fn add_link(&mut self, img_index: &Index, link: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.add_link(link);
        let output = Self::build_string(&img.links);
        let _ = self.exif.as_mut().unwrap().set_link(&img.file, &output);
    }

    pub fn del_artist(&mut self, img_index: &Index, artist: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];

        img.remove_artist(artist);
        Self::rem_taglist(&mut self.taglist, img_index, artist);

        let output = Self::build_string(&img.artists);
        let _ = self.exif.as_mut().unwrap().set_artist(&img.file, &output);
    }

    pub fn add_artist(&mut self, img_index: &Index, artist: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];

        img.add_artist(artist);
        Self::add_taglist(&mut self.taglist, img_index, artist);
        
        let output = Self::build_string(&img.artists);
        let _ = self.exif.as_mut().unwrap().set_artist(&img.file, &output);
    }
}
