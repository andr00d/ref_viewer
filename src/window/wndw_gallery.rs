use std::time::Instant;
use std::vec::Vec;
use eframe::egui::{self, Button};
use egui_extras::{TableBuilder, Column};

use crate::data::image::{Status, Index};
use crate::shared::{Shared, Textbox};
use crate::data::Data;

/////////////////////////

fn search_bar(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    let old_index = Index{folder: data_shared.main_img.folder, image: data_shared.main_img.image};

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
}

fn calc_table_dims (width: f32, icon_size: f32, img_data: &Data, data_shared: &Shared) -> (usize, Vec<f32>)
{
    let mut row_heights = Vec::new();
    let columns = (width / icon_size).floor() as usize; //TODO: remove as keyword. use the better option

    for (f, folder) in img_data.folders.iter().enumerate()
    {
        row_heights.push(30.0);
        if folder.collapsed {continue;}

        let rows = (data_shared.results[f].len() + columns - 1) / columns;
        for _i in 0..rows
        {
            row_heights.push(icon_size);
        }
    }

    return (columns, row_heights);
}

fn get_indexes (row: usize, columns: usize, img_data: &Data) -> (bool, Vec<Index>)
{
    let mut f = 0;
    let mut i = row;
    let mut indexes = Vec::<Index>::new();

    for folder in &img_data.folders
    {   
        if i == 0
        {
            indexes.push(Index{folder:f, image:0});
            return (true, indexes);
        } 

        if folder.collapsed
        {
            f += 1;
            i -= 1;
            continue;
        }

        i -= 1;
        let folder_rows = (folder.images.len() + columns - 1) / columns;

        if folder_rows <= i
        {
            f += 1;
            i -= folder_rows;
            continue;
        }
        else
        {
            break;
        }
    }

    let folder = &img_data.folders[f];
    let imgs_start = i*columns;
    let imgs_row = std::cmp::min(columns, folder.images.len() - imgs_start);

    for j in imgs_start..imgs_start + imgs_row
    {
        indexes.push(Index{folder:f, image:j});
    }

    return (false, indexes);
}

/////////////////////////

pub fn wndw_gallery(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{

    egui::CentralPanel::default().show(ui, |ui| {
        search_bar(ui, img_data, data_shared);

        let icon_size = 100.0;
        let padded_size = icon_size + ui.style_mut().spacing.item_spacing.x;
        let width = ui.available_width();
        let mut folder_to_close = None;

        let (columns, row_heights) = calc_table_dims(width, padded_size, img_data, data_shared);

        TableBuilder::new(ui)
        .columns(Column::remainder(), columns)
        .body(|body| {
            body.heterogeneous_rows(row_heights.into_iter(), |mut row| {
                let (is_folder, indexes) = get_indexes(row.index(), columns, img_data);

                if is_folder
                {
                    let folder = &mut img_data.folders[indexes[0].folder];
                    row.col(|ui| {
                        if ui.add_sized([icon_size, 30.0], Button::new(folder.btn_path.clone())).clicked()
                        {
                            folder_to_close = Some(indexes[0].folder);
                        }
                    });
                    return;
                }

                for index in &indexes
                {
                    row.col(|ui| {
                        
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
                                    data_shared.main_img = indexes[0].clone();
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
                }
                    
            });
        });

        if folder_to_close.is_some()
        {
            let folder = &mut img_data.folders[folder_to_close.unwrap()];
            folder.collapsed = !folder.collapsed;
        }
    });
}