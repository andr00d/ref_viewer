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

pub struct Shared
{
    pub main_img: Index,
    pub active_input: Textbox,
    pub last_update: Instant,
    pub frame_index: usize,
}