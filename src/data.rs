mod exiftool;
pub mod image;
pub mod data;

use std::collections::HashMap;

use crate::data::image::Image;
use crate::data::exiftool::Exiftool;
use crate::data::image::Index;

/////////////////////////

pub struct Folder 
{
    pub path: String,
    pub btn_path: String,
    pub collapsed: bool,
    pub images: Vec<Image>,
}

pub struct Data 
{
    exif: Option<Exiftool>, 
    pub folders: Vec<Folder>,
    pub taglist: HashMap::<String, Vec<Index>>,
}
