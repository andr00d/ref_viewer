use eframe::egui;
use egui::widget_text::RichText;
use crate::data::Data;
use crate::image::Index;

pub struct WndwRight
{
    pub artist: String,
    pub link: String,
    pub tag: String,
}

pub enum VectorType
{
    Tag,
    Link,
    Artist,
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

fn display_vector(ui: &mut egui::Ui, vector: &Vec<String>, textbox: &mut String, tagtype: VectorType) -> Option<Action>
{
    let mut result = None;

    for item in vector
    {
        let resp_del = match tagtype
        {
            VectorType::Link => ui.add(egui::Label::new(item)
                                  .sense(egui::Sense::click())),
            _ => ui.add(egui::Label::new("+ - ".to_string() + item)
                   .sense(egui::Sense::click())),
        };
    
        resp_del.context_menu(|ui| {
            if ui.button("Delete item").clicked() 
            {
                let _ = match tagtype
                {
                    VectorType::Tag => result.insert(Action::TagDel(item.clone())),
                    VectorType::Link => result.insert(Action::LinkDel(item.clone())),
                    VectorType::Artist => result.insert(Action::ArtistDel(item.clone())),
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
        let _ = match tagtype
        {
            VectorType::Tag => result.insert(Action::TagAdd(textbox.clone())),
            VectorType::Link => result.insert(Action::LinkAdd(textbox.clone())),
            VectorType::Artist => result.insert(Action::ArtistAdd(textbox.clone())),
        };
        textbox.clear();
        //TODO: keep focus after pressing enter
    }

    return result;
}

pub fn wndw_right(ui: &egui::Context, img_data: &mut Data, main_img: &Index, boxes: &mut WndwRight) -> ()
{
    let img = &img_data.folders[main_img.folder].images[main_img.image];
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
                
                let artist = display_vector(ui, &img.artists, &mut boxes.artist, VectorType::Artist);

                if !artist.is_none() && tag_action.is_none()
                {
                    tag_action = artist;
                }

                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("source")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));

                let link = display_vector(ui, &img.links, &mut boxes.link, VectorType::Link);

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
                
                let tag = display_vector(ui, &img.tags, &mut boxes.tag, VectorType::Tag);
                
                if !tag.is_none() && tag_action.is_none()
                {
                    tag_action = tag;
                }
            });
        });

    if tag_action.is_none() {return;}

    match tag_action.unwrap()
    {
        Action::TagAdd(x) => img_data.add_tag(main_img, &x),
        Action::TagDel(x) => img_data.del_tag(main_img, &x),
        Action::LinkAdd(x) => img_data.add_link(main_img, &x),
        Action::LinkDel(x) => img_data.del_link(main_img, &x),
        Action::ArtistAdd(x) => img_data.add_artist(main_img, &x),
        Action::ArtistDel(x) => img_data.del_artist(main_img, &x),
    };
}