use std::collections::HashMap;
use std::path::Path;
use core::cmp::Ordering;
use serde_json::Value;
use serde_json::json;

use crate::data::{Data, Folder, Image};
use crate::data::exiftool::Exiftool;
use crate::data::image::Index;
use crate::shared::Shared;

impl Data 
{
    pub fn new() -> Result<Data, String> 
    {
        let exif = Exiftool::new();
        let data = Vec::<Folder>::new();
        let taglist = HashMap::<String, Vec<Index>>::new();

        match exif
        {
            Some(x) => return Ok(Data {folders:data, exif:x, taglist:taglist}),
            None => return Err("exif not found on system".to_owned()),
        }
    }

    ///////////////////
    // data building //
    ///////////////////

    fn recurse(&self, path: &Path) -> Vec<String> 
    {
        let Ok(entries) = std::fs::read_dir(path) else { return vec![] };
        
        entries.flatten().flat_map(|entry| {
            let Ok(meta) = entry.metadata() else { return vec![] };
            if meta.is_dir() 
            { 
                let mut paths = self.recurse(entry.path().as_path());
                paths.insert(0, entry.path().to_str().unwrap().to_string());
                return paths; 
            }
            vec![]
        }).collect()
    }

    fn seperate_folders(&self, paths: &Vec::<String>) -> Vec::<String>
    {
        let mut folders = Vec::<String>::new();

        for folder in paths
        {
            folders.push(folder.clone());
            folders.append(&mut self.recurse(Path::new(folder)));
        }

        folders.sort_unstable();
        folders.dedup();
        return folders;
    }

    pub fn close_folder(&mut self, data_shared: &mut Shared, path: String)
    {
        if let Some(i) = self.folders.iter().position(|f| f.path == path)
        {
            self.folders.remove(i);
        };

        data_shared.update_search(self);
        self.build_tags();
    }

    pub fn open_folders(&mut self, paths: Vec::<String>) -> Option<Index>
    {
        let mut index = None;
        for input_path in self.seperate_folders(&paths)
        {
            let mut path = Path::new(&input_path);
            let is_file = path.is_file();
            if is_file {path = path.parent().unwrap();}

            if path.to_str().is_none()
            {
                let err_path = path.to_string_lossy().into_owned();
                println!("Error decoding the following path:");
                println!("{}", err_path);
                continue;
            }

            let str_path = path.to_str().unwrap().to_owned();
            match self.set_folder_data(&str_path)
            {
                Err(x) => println!("{}", x),
                Ok(_) => (),
            };

            if is_file 
            {
                index = Some(self.get_path_index(&input_path)
                        .unwrap_or(Index{folder:0, image:0}));
            }
        }

        self.build_tags();
        return index;
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

        #[cfg(windows)]
        let file = file.replace("/", "\\");
        
        let str_artists = info.get("Artist").unwrap_or(empty).as_str().unwrap_or("");
        let str_links = info.get("PageName").unwrap_or(empty).as_str().unwrap_or("");
        let str_tags = info.get("ImageDescription").unwrap_or(empty).as_str().unwrap_or("");
        let str_notes = info.get("UserComment").unwrap_or(empty).as_str().unwrap_or("");
        let size = info.get("ImageSize").unwrap_or(empty).as_str().unwrap_or("");
        
        let artists = Self::read_json(&str_artists.to_string());
        let links = Self::read_json(&str_links.to_string());
        let tags = Self::read_json(&str_tags.to_string());
        let notes = str_notes.replace("\\\"", "\"");


        let image = Image::new(
            file.to_string(), 
            artists, 
            size.to_string(), 
            links, 
            tags,
            notes,
        );

        return Ok(image)
    }

    // TODO: test non-existant folder
    fn set_folder_data(&mut self, path: &String) ->  Result<(), String>
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

        let output = match self.exif.get_folder_data(path)
        {
            Ok(x) => x,
            Err(_x) => return Err("Error with exiftool".to_string()),
        };

        if output.len() == 0 
        {
            self.folders.push(folder);
            return Ok(());
        }

        let json = match serde_json::from_str::<Value>(&output)
        {
            Ok(x) => x,
            Err(_x) => return Err("Error with json output".to_string()),
        };

        for value in json.as_array().unwrap()
        {
            match Self::construct_image(value)
            {
                Ok(x) => folder.images.push(x),
                Err(_x) => println!("error with image"),
            };
        }

        self.folders.push(folder);
        return Ok(());
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
    
    fn get_path_index(&self, path: &String) -> Option<Index>
    {
        let img_folder = Path::new(path).parent().unwrap();
        for (f, folder) in self.folders.iter().enumerate()
        {
            if Path::new(&folder.path) != img_folder {continue;}

            for (i, image) in folder.images.iter().enumerate()
            {
                if image.file == *path {return Some(Index{folder:f, image:i});}
            }
        }
        return None;
    }

    //////////////
    // taglist //
    /////////////

    fn build_tags(&mut self)
    {
        self.taglist.clear();

        for (f, folder) in self.folders.iter().enumerate()
        {
            for (i, image) in folder.images.iter().enumerate()
            {
                let index = Index{folder:f, image:i};
                for tag in &image.artists
                {
                    self.taglist.entry(tag.clone()).or_default().push(index.clone());  
                }
                for tag in &image.tags
                {
                    self.taglist.entry(tag.clone()).or_default().push(index.clone());  
                }
            }
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
        let _ = self.exif.set_tags(&img.file, &output);
    }

    pub fn add_tag(&mut self, img_index: &Index, tag: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        
        img.add_tag(tag);
        Self::add_taglist(&mut self.taglist, img_index, tag);

        let output = Self::build_string(&img.tags);
        let _ = self.exif.set_tags(&img.file, &output);
    }

    pub fn del_link(&mut self, img_index: &Index, link: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.remove_link(link);
        let output = Self::build_string(&img.links);
        let _ = self.exif.set_link(&img.file, &output);
    }

    pub fn add_link(&mut self, img_index: &Index, link: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.add_link(link);
        let output = Self::build_string(&img.links);
        let _ = self.exif.set_link(&img.file, &output);
    }

    pub fn del_artist(&mut self, img_index: &Index, artist: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];

        img.remove_artist(artist);
        Self::rem_taglist(&mut self.taglist, img_index, artist);

        let output = Self::build_string(&img.artists);
        let _ = self.exif.set_artist(&img.file, &output);
    }

    pub fn add_artist(&mut self, img_index: &Index, artist: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];

        img.add_artist(artist);
        Self::add_taglist(&mut self.taglist, img_index, artist);
        
        let output = Self::build_string(&img.artists);
        let _ = self.exif.set_artist(&img.file, &output);
    }

    pub fn set_notes(&mut self, img_index: &Index) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        let altered_notes = img.notes.replace("\"", "\\\"");
        let _ = self.exif.set_notes(&img.file, &altered_notes);
    }
}
