use std::time::Instant;
use eframe::egui::{self, Button};
use egui_extras::{TableBuilder, Column};

use crate::data::image::{Status, Index};
use crate::shared::{Shared, Textbox};
use crate::data::Data;

/////////////////////////

pub fn wndw_left(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    let old_index = Index{folder: data_shared.main_img.folder, image: data_shared.main_img.image};

    egui::SidePanel::left("left_panel")
    .exact_width(100.0)
    .resizable(false)
    .show(ui, |ui| {
        
        let resp_search = ui.add(egui::TextEdit::singleline(&mut data_shared.search).hint_text("search tags"));
        ui.add(egui::Separator::default());


        if resp_search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
        {
            let mut tags: Vec<String> = data_shared.search.split_whitespace().map(str::to_string).collect();
            let mut itags = tags.clone();

            tags.retain(|x| !x.starts_with("-"));
            itags.retain(|x| x.starts_with("-"));
            for part in &mut itags{part.remove(0);}

            data_shared.results = img_data.build_vector(tags, itags);
            if !data_shared.results[old_index.folder].contains(&old_index) && data_shared.results.len() > 0
            {
                data_shared.main_img  = Index{folder: 0, image: 0};
                for folder in &data_shared.results { if folder.len() > 0 {data_shared.main_img = folder[0].clone();} }
            }
        }

        if resp_search.gained_focus(){data_shared.active_input = Textbox::Search;}
        if data_shared.active_input == Textbox::Search {resp_search.request_focus();}
        
        let icon_size = 100.0;
        let mut row_heights = Vec::new();
        for (f, folder) in img_data.folders.iter().enumerate()
        {
            row_heights.push(30.0);
            if !folder.collapsed {for _i in 0..data_shared.results[f].len() {row_heights.push(100.0);}}
        }

        // use table instead of display_rows to allow for different row heights
        TableBuilder::new(ui)
        .column(Column::remainder().at_least(100.0))
        .body(|body| {
            body.heterogeneous_rows(row_heights.into_iter(), |mut row| {
                let mut i = row.index();
                let mut f = 0;

                row.col(|ui| {
                    
                    for folder in &mut img_data.folders
                    {   
                        if i == 0
                        {
                            if ui.add_sized([icon_size, 30.0], Button::new(folder.btn_path.clone())).clicked()
                            {
                                folder.collapsed = !folder.collapsed;
                            }

                            return;
                        } 

                        i -= 1;
                        if !folder.collapsed && i < folder.images.len() {break;}
                        if !folder.collapsed {i -= folder.images.len();}
                        f += 1;
                    }

                    if f >= img_data.folders.len() {return;}
                    let index = &data_shared.results[f][i];
                    let image = &mut img_data.folders[index.folder].images[index.image];

                    match image.thumb_state()
                    {
                        Status::Unloaded => 
                        {
                            image.load_thumb(); 
                        }

                        Status::Loading =>
                        {
                            image.poll_thumb(ui); 
                            ui.add_sized([icon_size, icon_size], egui::widgets::Spinner::new());
                        }
                        
                        Status::Loaded => 
                        {
                            let texture = image.thumb_texture.clone().unwrap();

                            let img_response = 
                            ui.add_sized([icon_size, icon_size],
                                egui::Image::new(&texture)
                                    .sense(egui::Sense {
                                        click: (true),
                                        drag: (true),
                                        focusable: (true),
                                    }),
                            );

                            if img_response.clicked()
                            {
                                data_shared.main_img = index.clone();
                                data_shared.last_update = Instant::now();
                                data_shared.frame_index = 0;
                            }
                        }

                        Status::Error => 
                        { 
                            let msg = "error loading ".to_string() + &image.file;
                            ui.add_sized([100.0, 100.0],
                                egui::Label::new(&msg)
                            ); 
                        }
                    }
                });
            });
        });
    });

    if old_index != data_shared.main_img
    {
        img_data.folders[old_index.folder].images[old_index.image].clear_full();
    }
}