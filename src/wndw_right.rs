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

pub enum Action
{
    TagAdd,
    TagDel,
    LinkAdd,
    LinkDel,
    ArtistAdd,
    ArtistDel,
}

pub fn wndw_right(ui: &egui::Context, img_data: &mut Data, main_img: &Index, boxes: &mut WndwRight ) -> ()
{
    let img = &img_data.folders[main_img.folder].images[main_img.image];
    let mut selected_data = "".to_string();
    let mut tag_action = None;

    egui::SidePanel::right("right_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

                ui.add(egui::Label::new(RichText::new("filepath")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                        
                let path = img.file.clone();
                ui.add(egui::Label::new(path));
                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("artist")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                
                match img.artist.clone().as_str()
                {
                    "" => ui.add(egui::Label::new("Unknown")),
                    other => ui.add(egui::Label::new(other)),
                };

                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("source")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));
                
                match img.link.clone().as_str()
                {
                    "" => ui.add(egui::Label::new("Unknown")),
                    other => ui.add(egui::Label::new(other)),
                };

                ui.add(egui::Separator::default());
                ui.add(egui::Separator::default());

                /////////////////////////////////////////

                ui.add(egui::Label::new(RichText::new("tags")
                        .background_color(egui::Color32::from_black_alpha(100))
                        .size(10.0)));

                for tag in &img.tags
                {
                    // TODO: add ability to delete tags
                    // TODO: add ability to copy tags
                    let resp_tag = ui.add(egui::Label::new("+ - ".to_string() + tag)
                                     .sense(egui::Sense::click()));
                
                    resp_tag.context_menu(|ui| {
                        if ui.button("Delete tag").clicked() 
                        {
                            let _ = tag_action.insert(Action::TagDel);
                            selected_data.push_str(tag);
                            ui.close_menu();
                        }
                    });
                }

                // TODO: add ability to add tags
                let resp_search = ui.add(
                        egui::TextEdit::singleline(&mut boxes.tag).hint_text("add tag")
                        .frame(false)
                    );
 
                if resp_search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
                {
                    let _ = tag_action.insert(Action::TagAdd);
                    selected_data = boxes.tag.clone();
                    boxes.tag.clear()
                }
            });
        });

    if tag_action.is_none() {return;}

    match tag_action.unwrap()
    {
        Action::TagAdd => img_data.add_tag(main_img, &selected_data),
        Action::TagDel => img_data.del_tag(main_img, &selected_data),
        Action::LinkAdd => (),
        Action::LinkDel => (),
        Action::ArtistAdd => (),
        Action::ArtistDel => (),
    };
}