use std::time::Instant;
use eframe::egui::{self, Button};
use crate::image::{Status, Index};
use crate::shared::Shared;
use crate::data::Data;
use crate::Textbox;

pub struct WndwLeft
{
    pub search: String,
    pub results: Vec<Index>,
}


pub fn wndw_left(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared, data: &mut WndwLeft) -> ()
{
    let old_index = Index{folder: data_shared.main_img.folder, image: data_shared.main_img.image};

    egui::SidePanel::left("left_panel")
    .exact_width(100.0)
    .resizable(false)
    .show(ui, |ui| {
        
        let resp_search = ui.add(egui::TextEdit::singleline(&mut data.search).hint_text("search tags"));
        ui.add(egui::Separator::default());


        if resp_search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
        {
            let mut tags: Vec<String> = data.search.split_whitespace().map(str::to_string).collect();
            let mut itags = tags.clone();

            tags.retain(|x| !x.starts_with("-"));
            itags.retain(|x| x.starts_with("-"));
            for part in &mut itags{part.remove(0);}

            data.results = img_data.build_vector(tags, itags);
            if !data.results.contains(&old_index) && data.results.len() > 0
            {
                data_shared.main_img.folder = data.results[0].folder;
                data_shared.main_img.image = data.results[0].image;
            }
        }

        if resp_search.gained_focus(){data_shared.active_input = Textbox::Search;}
        if data_shared.active_input == Textbox::Search {resp_search.request_focus();}

        egui::ScrollArea::vertical().show(ui, |ui| {
            // TODO: loop only over subset images to speed up startup for large folders
            let mut imglist_index = 0;
            for (i_folder, folder) in img_data.folders.iter_mut().enumerate()
            {
                // make folders collabsible
                if ui.add_sized([100.0, 20.0], Button::new(folder.path.clone())).clicked()
                {
                    folder.collapsed = !folder.collapsed;
                }
                
                if folder.collapsed {continue;}

                // display folder images
                for i in imglist_index..data.results.len()
                {
                    if data.results[i].folder > i_folder {break;}
                    if data.results[i].folder < i_folder {imglist_index += 1; continue;}

                    let i_image = data.results[imglist_index].image;
                    let image = &mut folder.images[i_image];
                    imglist_index += 1;

                    match image.thumb_state()
                    {
                        Status::Unloaded => 
                        {
                            image.load_thumb(); 
                        }

                        Status::Loading =>
                        {
                            image.poll_thumb(ui); 
                            ui.spinner();
                        }
                        
                        Status::Loaded => 
                        {
                            let texture = image.thumb_texture.clone().unwrap();

                            let img_response = 
                            ui.add_sized([100.0, 100.0],
                                egui::Image::new(&texture)
                                    .sense(egui::Sense {
                                        click: (true),
                                        drag: (true),
                                        focusable: (true),
                                    }),
                            );

                            if img_response.clicked()
                            {
                                data_shared.main_img.folder = i_folder;
                                data_shared.main_img.image = i_image;
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
                }
            }
        });     
    });

    if old_index != data_shared.main_img
    {
        img_data.folders[old_index.folder].images[old_index.image].clear_full();
    }
}