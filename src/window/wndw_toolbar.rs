use eframe::egui;
use egui::menu;

use crate::data::Data;
use crate::shared::{Shared, Gallery};
use crate::data::image::Index;

fn open_paths() -> std::vec::Vec<String>
{
    let mut paths: std::vec::Vec<String> = Vec::new();
 
    if let Some(fd_paths) = rfd::FileDialog::new()
        .add_filter("image", &["jpg", "jpeg", "png", "webp", "gif"])
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

        for path in &paths
        {
            std::println!("{}", path);
        }
    }

    // TODO: check folder inside other folder
    return paths;
}

pub fn wndw_toolbar(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared ) -> ()
{
    egui::TopBottomPanel::top("my_panel").show(ui, |ui| {
        menu::bar(ui, |ui| {
            ui.add_space(7.0);
            ui.menu_button("File", |ui| {
                if ui.button("Open file").clicked() 
                {
                    let paths = open_paths();
                    if paths.len() == 0 {return;}

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
                    let imagelist = img_data.build_vector(Vec::new(), Vec::new());
                    data_shared.set_results(imagelist);
                    data_shared.set_selected(img_data, &index, &index);
                }
            });

            if ui.button("About").clicked() 
            {
                std::println!("todo");
            }
        });
    });
}