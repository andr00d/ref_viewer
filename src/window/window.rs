use egui::Key;

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
        let data_shared = Shared::new(imagelist, index);


        Self { 
            img_data: img_data,
            data_shared: data_shared,
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
        get_inputs(ui, &mut self.data_shared);
        handle_inputs(&mut self.img_data, &mut self.data_shared);
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

fn handle_inputs(img_data: &mut Data, data_shared: &mut Shared)
{
    if data_shared.key_event.is_none() {return;}

    let mut total_results = 0;
    for folder in &data_shared.results {total_results += folder.len();}

    match data_shared.key_event.unwrap()
    {
        Key::ArrowUp | Key::ArrowLeft =>
        {
            match data_shared.prev_result()
            {
                Some(x) => 
                {
                    img_data.folders[x.folder].images[x.image].clear_full();
                    data_shared.main_img = x.clone();
                },
                None => (),
            }
        },

        Key::ArrowDown | Key::ArrowRight =>
        {
            match data_shared.next_result()
            {
                Some(x) => 
                {
                    img_data.folders[x.folder].images[x.image].clear_full();
                    data_shared.main_img = x.clone();
                },
                None => (),
            }
        },

        Key::Escape =>
        {
            if data_shared.gallery_type == Gallery::LeftBar
            {
                data_shared.gallery_type = Gallery::Full;
            }
            println!("escape");
        },

        Key::Enter =>
        {
            println!("nope.");
        }

        _ => println!("unhandled keypress."),
    }

    data_shared.key_event=None;
}

fn get_inputs(ui: &egui::Context, data_shared: &mut Shared)
{
    // TODO: shift select multiple
    let valid_keys = [Key::ArrowDown, Key::ArrowLeft, Key::ArrowRight, Key::ArrowUp,
                      Key::Escape];

    for key in valid_keys
    {
        if ui.input(|i| i.key_pressed(key)) 
        {
            data_shared.key_event = Some(key);
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