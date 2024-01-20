mod wndw_left;
mod wndw_right;
mod wndw_main;
mod exiftool;
mod data;
mod image;

use std::env;
use eframe::egui;
use crate::data::Data;
use crate::image::Index;
use crate::wndw_right::WndwRight;

fn main() -> Result<(), eframe::Error> 
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                .with_inner_size([1000.0, 600.0])
                .with_min_inner_size([300.0, 150.0]),
            ..Default::default()
    };

    eframe::run_native(
        "ref finder",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<RefViewer>::default()
        }),
    )
}

struct RefViewer
{
    img_data: Data,
    main_img: Index,
    search: String,
    text_right: WndwRight,
}

impl Default for RefViewer{
    fn default() -> Self 
    {
        let mut folders: Vec<String> = env::args().collect();
        folders.remove(0);

        Self { 
            img_data: Data::new(folders),
            main_img: Index{folder: 0, image: 0},
            search: "".to_string(),
            text_right: WndwRight{ artist: "".to_string(), 
                                    link: "".to_string(), 
                                    tag: "".to_string()},
        }
    }
}

impl eframe::App for RefViewer{
    fn update(&mut self, ui: &egui::Context, _frame: &mut eframe::Frame) 
    {
        wndw_left::wndw_left(ui, &mut self.img_data, &mut self.main_img, &mut self.search);

        match self.img_data.get_nr_imgs()
        {
            0 => {wndw_main::wndw_main_empty(ui);}
            _ =>
            {
                wndw_right::wndw_right(ui, &mut self.img_data, &mut self.main_img, &mut self.text_right);
                wndw_main::wndw_main(ui, &mut self.img_data, &mut self.main_img);
            }
        }
    }
}