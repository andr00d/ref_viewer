#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod wndw_left;
mod wndw_right;
mod wndw_main;
mod shared;
mod exiftool;
mod data;
mod image;

use ::image::load_from_memory;
use ::image::RgbaImage;
use std::path::Path;
use std::time::Instant;
use eframe::egui;

use crate::data::Data;
use crate::image::Index;
use crate::wndw_right::WndwRight;
use crate::wndw_left::WndwLeft;
use crate::shared::{Shared, Textbox};

/////////////////////////

#[derive(Default)]
struct ErrorWindow{}

struct RefViewer
{
    img_data: Data,
    data_shared: Shared,
    data_left: WndwLeft,
    data_right: WndwRight,
}

/////////////////////////

impl RefViewer
{
    fn new(img_data: Data, img_path: Option<String>) -> Self 
    {
        let imagelist = img_data.build_vector(Vec::new(), Vec::new());
        let mut index = Index{folder: 0, image: 0};

        if img_path.is_some() 
        {
            index = img_data.get_string_index(img_path.unwrap()).unwrap_or(index);
        }

        Self { 
            img_data: img_data,
            data_shared: Shared{main_img: index,
                                active_input: Textbox::Search,
                                last_update: Instant::now(),
                                frame_index: 0},
            data_left: WndwLeft{search: "".to_string(),
                                results: imagelist},
            data_right: WndwRight{artist: "".to_string(), 
                                  link: "".to_string(), 
                                  tag: "".to_string()},
        }
    }
}

impl eframe::App for RefViewer
{
    fn update(&mut self, ui: &egui::Context, _frame: &mut eframe::Frame) 
    {
        wndw_left::wndw_left(ui, &mut self.img_data, &mut self.data_shared, &mut self.data_left);

        let mut total_results = 0;
        for folder in &self.data_left.results {total_results += folder.len();}
    
        if self.img_data.get_nr_imgs() == 0 || total_results == 0
        {
            wndw_main::wndw_main_empty(ui);
        }
        else
        {
            wndw_right::wndw_right(ui, &mut self.img_data, &mut self.data_shared, &mut self.data_right);
            wndw_main::wndw_main(ui, &mut self.img_data, &mut self.data_shared);
        }
    }
}

fn run_window(img_data: Data, img_path: Option<String>, options: eframe::NativeOptions) -> Result<(), eframe::Error> 
{
    eframe::run_native(
        "ref viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(RefViewer::new(img_data, img_path)))
        }),
    )
}

/////////////////////////

impl eframe::App for ErrorWindow
{
    fn update(&mut self, ui: &egui::Context, _frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ui, |ui| {
            let window_area = egui::Rect{min:ui.next_widget_position(), 
                max:ui.next_widget_position() + ui.available_size()};
    
            #[cfg(unix)]
            ui.put(window_area, egui::Label::new("Error with exiftool \n Make sure that exiftool is installed on this system".to_string())); 

            #[cfg(windows)]
            ui.put(window_area, egui::Label::new("Error with exiftool \n Make sure that exiftool is in the same folder as the executable".to_string())); 
        });
    }
}

fn run_error_window(options: eframe::NativeOptions) -> Result<(), eframe::Error> 
{
    eframe::run_native(
        "ref viewer",
        options,
        Box::new(|cc| {Ok(Box::<ErrorWindow>::default())}),
    )
}

//////////////////////////

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
