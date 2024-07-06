#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod window;
mod shared;
mod data;

use ::image::load_from_memory;
use ::image::RgbaImage;
use std::path::Path;
use eframe::egui;

use crate::data::Data;
use crate::window::{window::run_error_window, window::run_window};

/////////////////////////

fn create_database() -> (Data, Option<String>)
{
    let mut folders: Vec<String> = std::env::args().collect();
    folders.remove(0);
    let mut img_path = None;

    for item in &mut folders 
    {
        let path = Path::new(&item);
        if path.is_file() 
        {
            img_path = Some(item.clone());
            *item = path.parent().unwrap().to_string_lossy().into_owned();
        };
    }

    let img_data = Data::new(folders);
    return (img_data, img_path);
}

fn eframe_options(img: RgbaImage, w: u32, h: u32) -> eframe::NativeOptions
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                .with_inner_size([1000.0, 600.0])
                .with_min_inner_size([300.0, 150.0])
                .with_icon(egui::IconData 
                    { 
                        rgba: img.into_raw(), 
                        width: w, 
                        height: h,
                    }),
            ..Default::default()
    };

    return options;
}

fn load_icon() -> (RgbaImage, u32, u32)
{
    let icon = include_bytes!("../media/icon.png");
    let img = load_from_memory(icon).expect("icon not found.").to_rgba8();
    let (w, h) = img.dimensions();
    return (img, w, h);
}

/////////////////////////////

fn main() -> Result<(), eframe::Error> 
{
    let (img, w, h) = load_icon();
    let options = eframe_options(img, w, h);
    let (img_data, img_path) = create_database();

    if !img_data.exif_available() {run_error_window(options)}
    else {run_window(img_data, img_path, options)}
}
