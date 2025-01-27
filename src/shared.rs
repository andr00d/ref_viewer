pub mod shared;

use std::time::Instant;

use crate::data::image::Index;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Textbox
{
    Search,
    Link,
    Artist,
    Tag,
    Notes,
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
    pub active_input: Option<Textbox>,
    pub gallery_type: Gallery,
    pub last_update: Instant,
    pub frame_index: usize,
    pub search: String,
    pub key_event: Option<egui::Key>,
    pub show_popup_about: bool,
    pub show_popup_help: bool,
    selected: Vec<Index>,
    selected_tags: [Vec<(String, usize)>; 3],
    results: Vec<Vec<Index>>,
    results_len: usize,
}