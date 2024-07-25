use std::time::Instant;

use crate::window::{RefViewer, ErrorWindow};
use crate::shared::{Shared, Textbox, Gallery};
use crate::data::Data;
use crate::data::image::Index;
use crate::window::{WndwRight, wndw_right};
use crate::window::wndw_toolbar;
use crate::window::wndw_main;
use crate::window::wndw_gallery;


impl RefViewer
{
    fn new(img_data: Data, index: Index) -> Self 
    {
        let imagelist = img_data.build_vector(Vec::new(), Vec::new());

        Self { 
            img_data: img_data,
            data_shared: Shared{main_img: index,
                                active_input: Textbox::Search,
                                gallery_type: Gallery::LeftBar,
                                last_update: Instant::now(),
                                frame_index: 0,
                                search: "".to_string(),
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
        wndw_toolbar::wndw_toolbar(ui, &mut self.img_data, &mut self.data_shared);
        
        if self.data_shared.gallery_type == Gallery::Full
        {
            wndw_right::wndw_right(ui, &mut self.img_data, &mut self.data_shared, &mut self.data_right);
            wndw_gallery::wndw_gallery(ui, &mut self.img_data, &mut self.data_shared);
        }
        else
        {
            wndw_gallery::wndw_left(ui, &mut self.img_data, &mut self.data_shared);
    
            let mut total_results = 0;
            for folder in &self.data_shared.results {total_results += folder.len();}
        
            if total_results == 0
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
}

pub fn run_window(img_data: Data, index: Index, options: eframe::NativeOptions) -> Result<(), eframe::Error> 
{
    eframe::run_native(
        "ref viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(RefViewer::new(img_data, index)))
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

pub fn run_error_window(options: eframe::NativeOptions) -> Result<(), eframe::Error> 
{
    eframe::run_native(
        "ref viewer",
        options,
        Box::new(|_cc| {Ok(Box::<ErrorWindow>::default())}),
    )
}