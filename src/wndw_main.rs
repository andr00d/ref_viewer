use std::time::{Instant, Duration};
use eframe::egui;
use eframe::egui::Vec2;
use crate::data::Data;
use crate::image::{Image, Status};
use crate::shared::Shared;


fn calc_scale(ui: &mut egui::Ui, img: &mut Image) -> egui::Vec2
{
    let img_size = img.full_texture[0].image.size_vec2();
    let ui_size = ui.available_size();
    let scale = &mut img.full_scale;

    match ui.input(|i| i.zoom_delta())
    {
        x if x == 1.0 => (),
        scroll =>
        {
            if scale.is_none()
            {
                let x = if img_size.x > ui_size.x {ui_size.x} else {img_size.x};
                let _ = scale.insert(x / img_size.x); 
            }
            else
            {
                match scroll 
                {
                    // make scaling non-linear to better handle extreme scaling?
                    x if x < 1.0  => scale.insert(f32::max(0.05, scale.unwrap() - 0.05)),
                    _             => scale.insert(f32::min(50.0, scale.unwrap() + 0.05)),
                };

            };
        },
    }


    match scale
    {
        Some(s) =>
        {
            let x = img_size.x * *s;
            let y = img_size.y * *s;
            return Vec2{x:x, y:y};
        },
        None => 
        {
            let x = if img_size.x > ui_size.x {ui_size.x} else {img_size.x};
            let y = if img_size.y > ui_size.y {ui_size.y} else {img_size.y};
            return Vec2{x:x, y:y};
        }
    }
}



pub fn wndw_main_empty(ui: &egui::Context) -> ()
{
    egui::CentralPanel::default().show(ui, |ui| {
        let window_area = egui::Rect{min:ui.next_widget_position(), 
            max:ui.next_widget_position() + ui.available_size()};

        ui.put(window_area, egui::Label::new("no images found".to_string())); 
    });
}

pub fn wndw_main(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    let img = &mut img_data.folders[data_shared.main_img.folder].images[data_shared.main_img.image];

    egui::CentralPanel::default().show(ui, |ui| {

        egui::ScrollArea::both().show(ui, |ui| {
            let window_area = egui::Rect{min:ui.next_widget_position(), 
                  max:ui.next_widget_position() + ui.available_size()};


            match img.full_state()
            {
                Status::Unloaded => 
                {
                    img.load_full(); 
                }

                Status::Loading =>
                {
                    img.poll_full(ui); 
                    ui.put(window_area, egui::widgets::Spinner::new());
                }
                
                Status::Loaded => 
                {
                    let texture = match img.full_texture.len()
                    {
                        1 => img.full_texture[0].image.clone(),
                        _ => 
                        {
                            let delay = img.full_texture[data_shared.frame_index].delay;
                            
                            if  Instant::now().duration_since(data_shared.last_update).as_millis() > delay.into()
                            {
                                data_shared.frame_index = (data_shared.frame_index + 1) % img.full_texture.len();
                                data_shared.last_update = Instant::now();
                                ui.ctx().request_repaint();
                            }
                            else
                            {
                                ui.ctx().request_repaint_after(Duration::from_millis(delay.into()));
                            }

                            img.full_texture[data_shared.frame_index].image.clone()
                        },
                    };

                    let scale = calc_scale(ui, img);
                    let ui_size = ui.available_size();
                    
                    let x = if scale.x > ui_size.x {scale.x} else {ui_size.x};
                    let y = if scale.y > ui_size.y {scale.y} else {ui_size.y};
                    let window_area = egui::Rect{min:ui.next_widget_position(), 
                        max:ui.next_widget_position() + Vec2{x:x, y:y}};

                    ui.put(window_area, egui::Image::new(&texture)
                            .fit_to_exact_size(scale));

                }

                Status::Error => 
                { 
                    let msg = "error loading ".to_string() + &img.file;
                    ui.put(window_area, egui::Label::new(&msg));
                }
            }
        });
    });
}