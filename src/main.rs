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
use crate::wndw_left::WndwLeft;

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
    data_left: WndwLeft,
    data_right: WndwRight,
}

impl Default for RefViewer{
    fn default() -> Self 
    {
        let mut folders: Vec<String> = env::args().collect();
        folders.remove(0);

        let img_data = Data::new(folders);
        let imagelist = img_data.build_vector(Vec::new(), Vec::new());

        Self { 
            img_data: img_data,
            main_img: Index{folder: 0, image: 0},
            data_left: WndwLeft{search: "".to_string(),
                                results: imagelist},
            data_right: WndwRight{ artist: "".to_string(), 
                                    link: "".to_string(), 
                                    tag: "".to_string()},
        }
    }
}

impl eframe::App for RefViewer{
    fn update(&mut self, ui: &egui::Context, _frame: &mut eframe::Frame) 
    {
        wndw_left::wndw_left(ui, &mut self.img_data, &mut self.main_img, &mut self.data_left);
    
        if self.img_data.get_nr_imgs() == 0 || self.data_left.results.len() == 0
        {
            wndw_main::wndw_main_empty(ui);
        }
        else
        {
            wndw_right::wndw_right(ui, &mut self.img_data, &mut self.main_img, &mut self.data_right);
            wndw_main::wndw_main(ui, &mut self.img_data, &mut self.main_img);
        }
    }
}