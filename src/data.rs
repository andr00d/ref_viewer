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

    fn construct_image(info : &serde_json::value::Value) -> Result<Image, &str>
    {
        let empty = &json!("");
        let vec_empty = Vec::<Value>::new();

        let file = info.get("SourceFile").unwrap_or(empty).as_str().unwrap_or("");
        if file.len() == 0 {return Err("filename incorrect")};
        
        let artist = info.get("Artist").unwrap_or(empty).as_str().unwrap_or("");
        let size = info.get("ImageSize").unwrap_or(empty).as_str().unwrap_or("");
        let link = info.get("PageName").unwrap_or(empty).as_str().unwrap_or("");
        let cmnt = info.get("UserComment").unwrap_or(empty).as_str().unwrap_or("");
        
        let json = serde_json::from_str::<Value>(cmnt).unwrap_or(empty.clone());
        let json = json.as_array().unwrap_or(&vec_empty);
        let mut tags = Vec::<String>::new();

        for value in json
        {
            if value.is_string() {tags.push(value.as_str().unwrap().to_string());}
        }

        let image = Image::new(
            file.to_string(), 
            artist.to_string(), 
            size.to_string(), 
            link.to_string(), 
            tags.clone()
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

    pub fn del_tag(&mut self, img_index: &Index, tag: &String) -> ()
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.remove_tag(tag);
        let mut comment = "[".to_string();

        for tag in &img.tags
        {
            comment.push_str(format!("\"{tag}\",").as_str());
        }
        
        if img.tags.len() > 0 {comment.pop();}
        comment.push_str("]");

        let _ = self.exif.set_usercomment(&img.file, &comment);
    }

    pub fn add_tag(&mut self, img_index: &Index, tag: &String)
    {
        let img = &mut self.folders[img_index.folder].images[img_index.image];
        img.add_tag(tag);
        let mut comment = "[".to_string();

        for tag in &img.tags
        {
            comment.push_str(format!("\"{tag}\",").as_str());
        }
        
        println!("{}", img.tags.len());
        comment.pop();
        comment.push_str("]");

        self.exif.set_usercomment(&img.file, &comment);
    }
}
