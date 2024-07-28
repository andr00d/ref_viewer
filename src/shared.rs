pub mod shared;

use std::time::Instant;

use crate::data::image::Index;

// data shared between multiple windows

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Textbox
{
    Search,
    Link,
    Artist,
    Tag,
}

#[derive(PartialEq)]
pub enum Gallery
{
    LeftBar,
    Full,
}

pub struct Shared
{
    pub main_img: Index,
    pub active_input: Textbox,
    pub gallery_type: Gallery,
    pub last_update: Instant,
    pub frame_index: usize,
    pub search: String,
    pub key_event: Option<egui::Key>,
    pub results: Vec<Vec<Index>>,
}