#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod window;
mod shared;
mod data;

use ::image::load_from_memory;
use ::image::RgbaImage;
use eframe::egui;

use crate::data::Data;
use crate::data::image::Index;
use crate::window::{window::run_error_window, window::run_window};

/////////////////////////

fn get_paths() -> Vec<String>
{
    let mut paths: Vec<String> = std::env::args().collect();
    paths.remove(0); // remove program from folder list
    return paths;
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
    let input_paths = get_paths();

    match Data::new()
    {
        Ok(mut x) => 
        {
            let index = x.open_paths(input_paths)
                .unwrap_or(Index{folder:0, image:0});
            return run_window(x, index, options)
        },

        Err(_) => return run_error_window(options),
    };
}
