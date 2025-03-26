use std::time::Instant;
use std::vec::Vec;
use regex::Regex;
use egui::Color32;
use eframe::egui::{self, Button};
use egui_extras::{TableBuilder, Column};

use crate::data::image::{Status, Index};
use crate::shared::{Shared, Gallery, Textbox};
use crate::data::Data;

const ICON_SIZE: f32 = 100.0;

/////////////////////////

fn search_bar(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    let resp_search = ui.add(egui::TextEdit::singleline(&mut data_shared.search).hint_text("search tags"));
    ui.add(egui::Separator::default());

    let re = Regex::new(r"[^a-zA-Z\d\s\-_*():]").unwrap();
    data_shared.search = re.replace_all(&data_shared.search, "").to_string();

    if resp_search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
    {
        data_shared.update_search(img_data);
    }

    if resp_search.gained_focus(){data_shared.active_input = Some(Textbox::Search);}
    if data_shared.active_input.is_none() {return;}
    if *data_shared.active_input.as_mut().unwrap() == Textbox::Search {resp_search.request_focus();}        
}

fn calc_table_dims (ui: &egui::Ui, img_data: &Data, data_shared: &Shared) -> (usize, Vec<f32>)
{
    let padded_size = ICON_SIZE + ui.style().spacing.item_spacing.x * 2.0;
    let columns = f32::max(1.0, (ui.available_width() / padded_size).floor()) as usize;
    let mut row_heights = Vec::new();

    for (f, folder) in img_data.folders.iter().enumerate()
    {
        row_heights.push(30.0);
        if folder.collapsed {continue;}

        let rows = (data_shared.get_results()[f].len() + columns - 1) / columns;
        for _i in 0..rows { row_heights.push(ICON_SIZE); }
    }

    return (columns, row_heights);
}

fn get_snap_index (ui: &egui::Ui, img_data: &Data, data_shared: &mut Shared) -> Option<usize>
{
    if !data_shared.snap_to_index {return None;}
    data_shared.snap_to_index = false;

    let padded_size = ICON_SIZE + ui.style().spacing.item_spacing.x * 2.0;
    let columns = f32::max(1.0, (ui.available_width() / padded_size).floor()) as usize;
    let mut index = 0;

    for (f, folder) in img_data.folders.iter().enumerate()
    {
        index += 1;
        if folder.collapsed {continue;}

        if f == data_shared.main_img.folder 
        {
            index += (data_shared.main_img.image + columns - 1) / columns;
            return Some(index);
        }
        else 
        {
            index += (data_shared.get_results()[f].len() + columns - 1) / columns;
        }
    }

    return None;
}

fn get_indexes (row: usize, columns: usize, img_data: &Data, data_shared: &Shared) -> (bool, Vec<Index>)
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
        let folder_size = data_shared.get_results()[f].len();
        let folder_rows = (folder_size + columns - 1) / columns;

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

    if f >= img_data.folders.len() {return (false, indexes);}
    let folder_size = data_shared.get_results()[f].len();
    let imgs_start = i*columns;
    let imgs_row = std::cmp::min(columns, folder_size - imgs_start);

    for j in imgs_start..imgs_start + imgs_row
    {
        indexes.push(data_shared.get_results()[f][j].clone());
    }

    return (false, indexes);
}

////////////////////////////


fn show_image(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared, index: &Index)
{
    let image = &mut img_data.folders[index.folder].images[index.image];
    let is_selected = data_shared.get_selected().contains(index);
    let texture = image.thumb_texture.clone().unwrap();

    let img_response = 
    ui.add_sized([ICON_SIZE, ICON_SIZE],
        egui::Button::image(&texture)
        .fill(Color32::TRANSPARENT)
        .selected(is_selected)
    );

    // ony allow multi selection in gallery mode
    if img_response.clicked() && ui.input(|i| i.modifiers.command_only()) &&
        data_shared.gallery_type == Gallery::Full
    {
        data_shared.add_selected(img_data, index);
    }

    // ony allow multi selection in gallery mode
    else if img_response.clicked() && ui.input(|i| i.modifiers.shift_only()) &&
        data_shared.gallery_type == Gallery::Full
    {
        let main_img = data_shared.main_img.clone();
        data_shared.set_selected(img_data, &main_img, index);
    }

    else if img_response.clicked()
    {
        if data_shared.main_img == *index
        {
            data_shared.gallery_type = Gallery::LeftBar;
        }
        else
        {
            data_shared.main_img = index.clone();
            data_shared.set_selected(img_data, index, index);
            data_shared.last_update = Instant::now();
            data_shared.frame_index = 0;
        }
    }
}

fn show_folder(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared, index: usize)
{
    let folder = &mut img_data.folders[index];
    let path = folder.path.clone();
    let resp = ui.add_sized([ICON_SIZE, 30.0], Button::new(folder.btn_path.clone())
                .sense(egui::Sense::click()));
    
    if resp.clicked() { folder.collapsed = !folder.collapsed; }

    resp.context_menu(|ui| {
        if ui.button("close folder").clicked() 
        {
            img_data.close_folder(data_shared, path);
            ui.close_menu();
        }
    }); 
}

////////////////////////////


fn show_gallery(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    search_bar(ui, img_data, data_shared);
    let (columns, row_heights) = calc_table_dims(ui, img_data, data_shared);
    let snap_index = get_snap_index(ui, img_data, data_shared);
    let mut table = TableBuilder::new(ui).columns(Column::remainder().at_least(ICON_SIZE), columns);
    
    if let Some(index) = snap_index {table = table.scroll_to_row(index, Some(egui::Align::Center));}

    table.body(|body| {
        body.heterogeneous_rows(row_heights.into_iter(), |mut row| {
            
            let row_index = row.index();
            let (is_folder, indexes) = get_indexes(row_index, columns, img_data, data_shared);

            if is_folder
            {
                row.col(|ui| {show_folder(ui, img_data, data_shared, indexes[0].folder)});
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
                            ui.add_sized([ICON_SIZE, ICON_SIZE], egui::widgets::Spinner::new());
                        }
                        
                        Status::Loaded => 
                        {
                            show_image(ui, img_data, data_shared, index);
                        }

                        Status::Error => 
                        { 
                            let msg = "error loading ".to_string() + &image.file;
                            ui.add_sized([ICON_SIZE,  ICON_SIZE], egui::Label::new(&msg)); 
                        }
                    }
                });
            }
                
        });
    });
}


pub fn wndw_gallery(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    egui::CentralPanel::default().show(ui, |ui| {
        show_gallery(ui, img_data, data_shared);       
    });
}

pub fn wndw_left(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    egui::SidePanel::left("left_panel")
    .exact_width(120.0)
    .max_width(120.0)
    .resizable(false)
    .show(ui, |ui| {
        show_gallery(ui, img_data, data_shared);       
    });
}
