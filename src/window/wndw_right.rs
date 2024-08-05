use eframe::egui;
use regex::Regex;
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
    ArtistAdd(String),
    ArtistDel(String),
    LinkAdd(String),
    LinkDel(String),
    TagAdd(String),
    TagDel(String),
    SearchAdd(String),
    SearchDel(String),
}

/////////////////////////

fn add_button(ui: &mut egui::Ui, text: &str) -> bool
{
    let size = egui::Vec2{x:15.0, y:0.0};
    let color = egui::Color32::TRANSPARENT;
    ui.spacing_mut().item_spacing = egui::vec2(0.0, 2.0);

    return ui.add(egui::Button::new(text).min_size(size).fill(color)).clicked() 
}

fn display_vector(ui: &mut egui::Ui, textbox: &mut String, data_shared: &mut Shared, boxtype: Textbox) -> Option<Action>
{
    let mut result = None;

    let vector = match boxtype
    {
        Textbox::Artist =>  &data_shared.get_selected_tags()[0],
        Textbox::Link => &data_shared.get_selected_tags()[1],
        Textbox::Tag => &data_shared.get_selected_tags()[2],
        _  => &data_shared.get_selected_tags()[0], // should never happen.
    };

    for (item, count) in vector
    {
        let resp_del = match boxtype
        {
            Textbox::Link => 
            {
                if *count == 1 {ui.add(egui::Label::new(format!("{item}")).sense(egui::Sense::click()))}
                else {ui.add(egui::Label::new(format!("({count}): {item}")).sense(egui::Sense::click()))}
            },
            _ => 
            {
                ui.horizontal(|ui| {
                    if add_button(ui, "+") {result = Some(Action::SearchAdd(item.clone()));}
                    if add_button(ui, "-") {result = Some(Action::SearchDel(item.clone()));}

                    if *count == 1 { ui.add(egui::Label::new(format!("{item}")).sense(egui::Sense::click()))}
                    else { ui.add(egui::Label::new(format!("({count}): {item}")).sense(egui::Sense::click()))}
                }).inner
            },
        };

        resp_del.context_menu(|ui| {
            if ui.button("Delete item").clicked() 
            {
                match boxtype
                {
                    Textbox::Artist => result = Some(Action::ArtistDel(item.clone())),
                    Textbox::Link => result = Some(Action::LinkDel(item.clone())),
                    Textbox::Tag => result = Some(Action::TagDel(item.clone())),
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
    let re = Regex::new(r"[^a-zA-Z\d_():]").unwrap();
    *textbox = re.replace_all(textbox, "").to_string();

    if resp_add.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
    {
        match boxtype
        {
            Textbox::Artist => result = Some(Action::ArtistAdd(textbox.clone())),
            Textbox::Link => result = Some(Action::LinkAdd(textbox.clone())),
            Textbox::Tag => result = Some(Action::TagAdd(textbox.clone())),
            _  => (),
        };
        textbox.clear();
    }

    let active = &mut data_shared.active_input;
    if resp_add.gained_focus() {*active = Some(boxtype.clone());}
    if active.is_some() && *active.as_mut().unwrap() == boxtype {resp_add.request_focus();}

    return result;
}

//////////////////////////////

fn info_main(ui: &mut egui::Ui, img_data: &mut Data, data_shared: &mut Shared, boxes: &mut WndwRight) -> Option<Action>
{
    let mut tag_action = None;
    let single_image = data_shared.get_selected().len() == 1;
    let img = &img_data.folders[data_shared.get_selected()[0].folder].images[data_shared.get_selected()[0].image];

    ui.add(egui::Label::new(RichText::new("filepath")
        .background_color(egui::Color32::from_black_alpha(100))
        .size(10.0)));
            
    if single_image {ui.add(egui::Label::new(&img.file));}
    else {ui.add(egui::Label::new("multiple images"));}
    ui.add(egui::Separator::default());


    /////////////////////////////////////////

    ui.add(egui::Label::new(RichText::new("size")
        .background_color(egui::Color32::from_black_alpha(100))
        .size(10.0)));
    
    if single_image {ui.add(egui::Label::new(&img.size));}
    else {ui.add(egui::Label::new("multiple images"));}
    ui.add(egui::Separator::default());

    /////////////////////////////////////////

    ui.add(egui::Label::new(RichText::new("artist")
        .background_color(egui::Color32::from_black_alpha(100))
        .size(10.0)));
    
    let artist = display_vector(ui, &mut boxes.artist, data_shared, Textbox::Artist);
    if !artist.is_none() && tag_action.is_none() {tag_action = artist;}

    ui.add(egui::Separator::default());

    /////////////////////////////////////////

    ui.add(egui::Label::new(RichText::new("source")
            .background_color(egui::Color32::from_black_alpha(100))
            .size(10.0)));

    let link = display_vector(ui, &mut boxes.link, data_shared, Textbox::Link);
    if !link.is_none() && tag_action.is_none() {tag_action = link;}

    ui.add(egui::Separator::default());
    ui.add(egui::Separator::default());

    /////////////////////////////////////////

    ui.add(egui::Label::new(RichText::new("tags")
            .background_color(egui::Color32::from_black_alpha(100))
            .size(10.0)));
    
    let tag = display_vector(ui, &mut boxes.tag, data_shared, Textbox::Tag);
    if !tag.is_none() && tag_action.is_none() {tag_action = tag;}

    return tag_action
}

pub fn wndw_right(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared, boxes: &mut WndwRight) -> ()
{

    egui::SidePanel::right("right_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ui, |ui| {
            match data_shared.get_selected().len()
            {
                0 => return,
                _ => 
                {
                    let action = info_main(ui, img_data, data_shared, boxes);
                    
                    if action.is_none() {return;}

                    match action.as_ref().unwrap()
                    {
                        Action::ArtistAdd(x) => for i in data_shared.get_selected() {img_data.add_artist(i, &x);},
                        Action::ArtistDel(x) => for i in data_shared.get_selected() {img_data.del_artist(i, &x);},
                        Action::LinkAdd(x) => for i in data_shared.get_selected() {img_data.add_link(i, &x);},
                        Action::LinkDel(x) => for i in data_shared.get_selected() {img_data.del_link(i, &x);},
                        Action::TagAdd(x) => for i in data_shared.get_selected() {img_data.add_tag(i, &x);},
                        Action::TagDel(x) => for i in data_shared.get_selected() {img_data.del_tag(i, &x);},
                        Action::SearchAdd(x) => data_shared.add_to_search(img_data, x),
                        Action::SearchDel(x) => data_shared.rem_from_search(img_data, x),
                    };

                    data_shared.update_tags(img_data);
                },
            }
        });
}