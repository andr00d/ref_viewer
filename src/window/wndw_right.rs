use eframe::egui;
use egui::widget_text::RichText;

use crate::data::Data;
use crate::shared::{Shared, Textbox};

/////////////////////////

pub struct WndwRight
{
    pub artist: String,
    pub link: String,
    pub tag: String,
}

pub enum Action
{
    TagAdd(String),
    TagDel(String),
    LinkAdd(String),
    LinkDel(String),
    ArtistAdd(String),
    ArtistDel(String),
}

/////////////////////////

fn display_vector(ui: &mut egui::Ui, vector: &Vec<String>, textbox: &mut String, active: &mut Option<Textbox>, boxtype: Textbox) -> Option<Action>
{
    let mut result = None;

    for item in vector
    {
        let resp_del = match boxtype
        {
            Textbox::Link => ui.add(egui::Label::new(item).sense(egui::Sense::click())),
            _ => ui.add(egui::Label::new(" +  -  ".to_string() + item).sense(egui::Sense::click())),
        };
    
        resp_del.context_menu(|ui| {
            if ui.button("Delete item").clicked() 
            {
                match boxtype
                {
                    Textbox::Tag => result = Some(Action::TagDel(item.clone())),
                    Textbox::Link => result = Some(Action::LinkDel(item.clone())),
                    Textbox::Artist => result = Some(Action::ArtistDel(item.clone())),
                    _  => (),
                };
                ui.close_menu();
            }
        }); 
    }

    let resp_add = ui.add(
            egui::TextEdit::singleline(textbox).hint_text("add item")
            .frame(false)
        );

    if resp_add.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
    {
        match boxtype
        {
            Textbox::Tag => result = Some(Action::TagAdd(textbox.clone())),
            Textbox::Link => result = Some(Action::LinkAdd(textbox.clone())),
            Textbox::Artist => result = Some(Action::ArtistAdd(textbox.clone())),
            _  => (),
        };
        textbox.clear();
    }

    if resp_add.gained_focus() {*active = Some(boxtype.clone());}
    if active.is_some() && *active.as_mut().unwrap() == boxtype {resp_add.request_focus();}

    return result;
}

pub fn wndw_right(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared, boxes: &mut WndwRight) -> ()
{
    let img = &img_data.folders[data_shared.main_img.folder].images[data_shared.main_img.image];
    let mut tag_action = None;

    egui::SidePanel::right("right_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

                ui.add(egui::Label::new(RichText::new("filepath")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                        
                ui.add(egui::Label::new(&img.file));
                ui.add(egui::Separator::default());


                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("size")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                
                ui.add(egui::Label::new(&img.size));
                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("artist")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                
                let artist = display_vector(ui, &img.artists, &mut boxes.artist, &mut data_shared.active_input, Textbox::Artist);

                if !artist.is_none() && tag_action.is_none()
                {
                    tag_action = artist;
                }

                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("source")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));

                let link = display_vector(ui, &img.links, &mut boxes.link, &mut data_shared.active_input, Textbox::Link);

                if !link.is_none() && tag_action.is_none()
                {
                    tag_action = link;
                }

                ui.add(egui::Separator::default());
                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("tags")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                
                let tag = display_vector(ui, &img.tags, &mut boxes.tag, &mut data_shared.active_input, Textbox::Tag);
                
                if !tag.is_none() && tag_action.is_none()
                {
                    tag_action = tag;
                }
            });
        });

    if tag_action.is_none() {return;}

    match tag_action.unwrap()
    {
        Action::TagAdd(x) => img_data.add_tag(&data_shared.main_img, &x),
        Action::TagDel(x) => img_data.del_tag(&data_shared.main_img, &x),
        Action::LinkAdd(x) => img_data.add_link(&data_shared.main_img, &x),
        Action::LinkDel(x) => img_data.del_link(&data_shared.main_img, &x),
        Action::ArtistAdd(x) => img_data.add_artist(&data_shared.main_img, &x),
        Action::ArtistDel(x) => img_data.del_artist(&data_shared.main_img, &x),
    };
}