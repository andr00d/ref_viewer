use eframe::egui::{self, Button};
use crate::image::{Status, Index};
use crate::data::Data;

pub fn wndw_left(ui: &egui::Context, img_data: &mut Data, main_index: &mut Index, search: &mut String ) -> ()
{
    let old_index = Index{folder: main_index.folder, image: main_index.image};

    egui::SidePanel::left("left_panel")
    .exact_width(100.0)
    .resizable(false)
    .show(ui, |ui| {
        
        
        let resp_search = ui.add(egui::TextEdit::singleline(search).hint_text("search tags"));
        ui.add(egui::Separator::default());


        if resp_search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
        {
            // not very good code. i'll clean it up later. 
            let mut tags: Vec<String> = search.split_whitespace().map(str::to_string).collect();
            let mut itags = tags.clone();
            tags.retain(|x| !x.starts_with("-"));
            itags.retain(|x| x.starts_with("-"));
            for part in &mut itags{part.remove(0);}

            for part in itags
            {
                println!("{}", part);
            }
        }


        egui::ScrollArea::vertical().show(ui, |ui| {
            // TODO: loop only over subset images to speed up startup for large folders
            for (i_folder, folder) in img_data.folders.iter_mut().enumerate()
            {
                // make folders collabsible
                if ui.add_sized([100.0, 20.0], Button::new(folder.path.clone())).clicked()
                {
                    folder.collapsed = !folder.collapsed;
                }
                
                if folder.collapsed {continue;}

                // display folder images
                for (i_image, image) in folder.images.iter_mut().enumerate()
                {
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