use eframe::egui;
use egui::menu;

use crate::data::Data;
use crate::shared::{Shared, Gallery};
use crate::data::image::Index;

fn open_folder() -> Vec<String>
{
    let mut paths: Vec<String> = Vec::new();
 
    if let Some(fd_paths) = rfd::FileDialog::new().pick_folders()
    {
        for path in fd_paths
        {
            match path.to_str()
            {
                Some(x) => paths.push(x.to_string()),
                None => (), //TODO: test not-utf8 path behaviour
            }
        }
    }

    return paths;
}

fn open_paths() -> Vec<String>
{
    let mut paths: Vec<String> = Vec::new();
 
    if let Some(fd_paths) = rfd::FileDialog::new()
        .add_filter("image", &["jpg", "jpeg", "png", "tga", "tiff", "webp", "gif"])
        .pick_files() 
    {
        for path in fd_paths
        {
            match path.to_str()
            {
                Some(x) => paths.push(x.to_string()),
                None => (), //TODO: test not-utf8 path behaviour
            }
        }
    }

    return paths;
}

fn update_data(img_data: &mut Data, data_shared: &mut Shared, paths: Vec<String>) -> ()
{
    // TODO: add popup about invalid paths. 
    match img_data.open_paths(paths)
    {
        Some(x) =>
        {
            data_shared.main_img = x;
            data_shared.gallery_type = Gallery::LeftBar;
        }
        None =>
        {
            data_shared.main_img = Index{folder:0, image: 0};
            data_shared.gallery_type = Gallery::Full;
        }
    }
    
    data_shared.search = "".to_string();
    let index = data_shared.main_img.clone();
    data_shared.update_search(img_data);
    data_shared.set_selected(img_data, &index, &index);
}

fn show_about(ui: &egui::Ui, data_shared: &mut Shared)
{
    let popup_size = egui::Vec2{x:500.0, y:300.0};
    let window_size = ui.ctx().screen_rect().max;
    let pos_min = (window_size - popup_size) / 2.0;
    let pos_max = pos_min + popup_size;

    egui::Window::new("About").title_bar(true).open(&mut data_shared.show_popup).fixed_size(popup_size)
    .default_rect(egui::Rect{min: pos_min, max: pos_max}).show(ui.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(egui::include_image!("../../media/icon_big.png"))
                   .fit_to_exact_size(egui::Vec2{x:200.0, y:200.0}));
            
            ui.vertical(|ui| {
                ui.heading("Ref Viewer");

                ui.add_space(12.0);
                ui.label(format!("V{}", env!("CARGO_PKG_VERSION")));
                ui.add_space(12.0);

                ui.label(
                    "Ref viewer is a simple image viewer that allows for tagging images and filtering based on those tags." 
                );

                ui.add_space(12.0);
                ui.label("Copyright © 2024 Andr00d");
                ui.label("Licensed under MIT.");
                ui.hyperlink_to("Ref viewer Github", "https://github.com/andr00d/ref_viewer/");
                ui.add_space(12.0);

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Uses ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" for the GUI.");
                });

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.hyperlink_to("Exiftool", "https://exiftool.org/");
                    ui.label(" is used for reading and writing tags.");
                });
            });
        });
     });
}

pub fn wndw_toolbar(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared ) -> ()
{
    egui::TopBottomPanel::top("my_panel").show(ui, |ui| {
        menu::bar(ui, |ui| {
            ui.add_space(7.0);
            ui.menu_button("File", |ui| {
                if ui.button("Open folder").clicked() 
                {
                    let paths = open_folder();
                    if paths.len() == 0 {return;}
                    update_data(img_data, data_shared, paths);
                }

                if ui.button("Open file").clicked() 
                {
                    let paths = open_paths();
                    if paths.len() == 0 {return;}
                    update_data(img_data, data_shared, paths);
                }
            });

            if ui.button("About").clicked() {data_shared.show_popup = !data_shared.show_popup;}
            if data_shared.show_popup {show_about(ui, data_shared);}
        });
    });
}