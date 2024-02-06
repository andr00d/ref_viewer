use eframe::egui::{self, Button};
use crate::image::{Status, Index};
use crate::data::Data;

pub struct WndwLeft
{
    pub search: String,
    pub results: Vec<Index>,
}


pub fn wndw_left(ui: &egui::Context, img_data: &mut Data, main_index: &mut Index, data: &mut WndwLeft) -> ()
{
    let old_index = Index{folder: main_index.folder, image: main_index.image};

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
                main_index.folder = data.results[0].folder;
                main_index.image = data.results[0].image;
            }
        }


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
                                main_index.folder = i_folder;
                                main_index.image = i_image;
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

    if old_index != *main_index
    {
        img_data.folders[old_index.folder].images[old_index.image].clear_full();
    }
}