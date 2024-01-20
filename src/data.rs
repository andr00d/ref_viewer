use serde_json::Value;
use serde_json::json;
use crate::image::{Image, Index};
use crate::exiftool::Exiftool;

pub struct Folder 
{
    pub path: String,
    pub collapsed: bool,
    pub images: Vec<Image>,
}

// TODO: implement tags
// pub struct TagList
// {
//     pub tag: String,
//     pub images: Vec<&Image>
// }

pub struct Data 
{
    exif: Exiftool, 
    pub folders: Vec<Folder>,
    // pub taglist: HashMap<TagList>,
}

impl Data 
{
    pub fn new(paths: Vec<String>) -> Data 
    {
        let mut exif = Exiftool::new();
        let mut data = Vec::<Folder>::new();
        for path in paths
        {
            let result = Self::get_folder_data(&path, &mut exif);
            match result
            {
                Ok(x) => data.push(x),
                Err(x) => println!("{}", x),
            };
        }

        Data { folders:data, exif:exif}
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
    fn get_folder_data(path: &String, exif: &mut Exiftool) ->  Result<Folder, String>
    {
        let mut folder = Folder{
            path: path.to_string(), 
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
        for value in json.as_array().unwrap()
        {
            match Self::construct_image(value)
            {
                Ok(x) => folder.images.push(x),
                Err(_x) => println!("error with image"),
            };
        }

        return Ok(folder);
    } 
    
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
        let output = Self::build_string(&img.tags);
        let _ = self.exif.set_usercomment(&img.file, &output);
    }

    pub fn add_tag(&mut self, img_index: &Index, tag: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.add_tag(tag);
        let output = Self::build_string(&img.tags);
        let _ = self.exif.set_usercomment(&img.file, &output);
    }

    pub fn del_link(&mut self, img_index: &Index, link: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.remove_link(link);
        let output = Self::build_string(&img.links);
        let _ = self.exif.set_link(&img.file, &output);
    }

    pub fn add_link(&mut self, img_index: &Index, link: &String)
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
        let output = Self::build_string(&img.artists);
        let _ = self.exif.set_artist(&img.file, &output);
    }

    pub fn add_artist(&mut self, img_index: &Index, artist: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.add_artist(artist);
        let output = Self::build_string(&img.artists);
        let _ = self.exif.set_artist(&img.file, &output);
    }
}
