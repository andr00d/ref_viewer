use egui::Key;

use crate::window::{RefViewer, ErrorWindow};
use crate::shared::{Shared, Gallery};
use crate::data::Data;
use crate::data::image::Index;
use crate::window::{WndwRight, wndw_right};
use crate::window::wndw_toolbar;
use crate::window::wndw_main;
use crate::window::wndw_gallery;


impl RefViewer
{
    fn new(mut img_data: Data, index: Index) -> Self 
    {
        let imagelist = img_data.build_vector(Vec::new(), Vec::new());
        let mut data_shared = Shared::new(imagelist, index.clone());
        
        if data_shared.get_result_size() > 0 
        {
            data_shared.set_selected(&mut img_data, &index, &index);
        }

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
        handle_inputs(ui, &mut self.img_data, &mut self.data_shared);
        wndw_toolbar::wndw_toolbar(ui, &mut self.img_data, &mut self.data_shared);
        
        if self.data_shared.gallery_type == Gallery::Full
        {
            if self.data_shared.get_result_size() > 0
            {
                wndw_right::wndw_right(ui, &mut self.img_data, &mut self.data_shared, &mut self.data_right);
            }
            wndw_gallery::wndw_gallery(ui, &mut self.img_data, &mut self.data_shared);
        }
        else
        {
            wndw_gallery::wndw_left(ui, &mut self.img_data, &mut self.data_shared);
    
            if self.data_shared.get_result_size() == 0
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

fn handle_inputs(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared)
{
    if data_shared.key_event.is_none() {return;}

    // TODO: handle up & down key better when in gallery mode.
    match data_shared.key_event.unwrap()
    {
        Key::ArrowUp | Key::ArrowLeft =>
        {
            match data_shared.prev_result(&data_shared.main_img)
            {
                Some(x) => 
                {
                    img_data.folders[x.folder].images[x.image].clear_full();
                    data_shared.main_img = x.clone();
                    data_shared.frame_index = 0;
                    data_shared.set_selected(img_data, &x, &x);
                    data_shared.snap_to_index = true;
                },
                None => (),
            }
        },

        Key::ArrowDown | Key::ArrowRight =>
        {
            match data_shared.next_result(&data_shared.main_img)
            {
                Some(x) => 
                {
                    img_data.folders[x.folder].images[x.image].clear_full();
                    data_shared.main_img = x.clone();
                    data_shared.frame_index = 0;
                    data_shared.set_selected(img_data, &x, &x);
                    data_shared.snap_to_index = true;
                },
                None => (),
            }
        },

        Key::Escape =>
        {
            if data_shared.active_input != None
            {
                data_shared.active_input = None;
            }

            else if data_shared.gallery_type == Gallery::LeftBar
            {
                data_shared.gallery_type = Gallery::Full;
            }

            else if data_shared.gallery_type == Gallery::Full
            {
                println!("exiting, thank you for using ref viewer!");
                ui.send_viewport_cmd(egui::viewport::ViewportCommand::Close);
            }
        },

        Key::Enter =>
        {
            if data_shared.gallery_type == Gallery::Full &&
               data_shared.active_input == None &&
               data_shared.get_result_size() > 0
            {
                data_shared.gallery_type = Gallery::LeftBar;
                let index = data_shared.main_img.clone();
                data_shared.set_selected(img_data, &index, &index);
            }
        }

        _ => println!("unhandled keypress."),
    }

    data_shared.key_event=None;
}

fn get_inputs(ui: &egui::Context, data_shared: &mut Shared)
{
    let valid_keys = [Key::ArrowDown, Key::ArrowLeft, Key::ArrowRight, Key::ArrowUp,
                      Key::Escape, Key::Enter];

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