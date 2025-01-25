use std::time::{Instant, Duration};
use eframe::egui;
use eframe::egui::Vec2;
use egui::TextureHandle;
use egui::emath::TSTransform;

use crate::data::image::{Image, Status};
use crate::shared::Shared;
use crate::data::Data;

fn get_frame(ui: &mut egui::Ui, img: &mut Image, data_shared: &mut Shared) -> TextureHandle
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

    return texture;
}

fn bounds_check(mut ts: TSTransform, img_size: Vec2, ui_size: Vec2, offset: Vec2) -> TSTransform
{

    let max_zoom = f32::max(4096.0, f32::min(img_size.x, img_size.y)) / f32::min(img_size.x, img_size.y);
    let min_zoom = f32::min(64.0, f32::max(img_size.x, img_size.y)) / f32::max(img_size.x, img_size.y);
    if ts.scaling < min_zoom {ts.scaling = min_zoom;}
    if ts.scaling > max_zoom {ts.scaling = max_zoom;}
    let scaled_size = img_size*ts.scaling;
    
    if ui_size.x > scaled_size.x { ts.translation.x = ((ui_size.x - scaled_size.x) / 2.0) + offset.x; }
    if ui_size.y > scaled_size.y { ts.translation.y = ((ui_size.y - scaled_size.y) / 2.0) + offset.y; }
    
    if scaled_size.x >= ui_size.x
    {
        let x_max = -scaled_size.x + ui_size.x + offset.x;
        if ts.translation.x > offset.x {ts.translation.x = offset.x;}
        if ts.translation.x < x_max {ts.translation.x = x_max;}
    }

    if scaled_size.y >= ui_size.y
    {
        let y_max = -scaled_size.y + ui_size.y + offset.y;
        if ts.translation.y > offset.y {ts.translation.y = offset.y;}
        if ts.translation.y < y_max {ts.translation.y = y_max;}
    }


    return ts;
}

fn calc_transform(ui: &mut egui::Ui, img: &mut Image) -> (TSTransform, bool)
{
    let mut interacted = false;
    let ui_size = ui.available_size();
    let img_size = img.full_texture[0].image.size_vec2();
    let offset = ui.next_widget_position().to_vec2();

    let mut transform = if img.transform == None
    {
        let mut ts = TSTransform::default();

        if img_size.x > ui_size.x || img_size.y > ui_size.y 
        {
            ts.scaling = f32::min(ui_size.x/img_size.x, ui_size.y/img_size.y);
        }

        ts.translation += offset;
        ts
    }
    else {img.transform.unwrap()};

    let (tid, rect) = ui.allocate_space(ui.available_size());
    let response = ui.interact(rect, tid, egui::Sense::click_and_drag());
    
    if response.dragged() 
    {
        transform.translation += response.drag_delta(); 
        interacted = true;
    }

    // thank you egui for letting me steal your example code
    if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) 
    {
        if ui.rect_contains_pointer(rect) 
        {
            let pointer_in_layer = transform.inverse() * pointer;
            let zoom_delta = ui.ctx().input(|i| i.zoom_delta());

            let max_zoom = f32::max(4096.0, f32::min(img_size.x, img_size.y)) / f32::min(img_size.x, img_size.y);
            if !(zoom_delta > 1.0 && transform.scaling >= max_zoom)
            {
                transform = transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());
            }

            if zoom_delta != 1.0 {interacted = true;}
        }
    }

    transform = bounds_check(transform, img_size, ui_size, offset);
    return (transform, interacted);
}

fn show_img_area(ui: &mut egui::Ui, texture: TextureHandle, transform: TSTransform)
{
    let size = texture.size_vec2().to_pos2() * transform.scaling;
    let min = transform.translation.to_pos2();
    let max = min + size.to_vec2();
    let rect = egui::Rect{min:min, max:max};

    // image should not capture responses, so use paint_at
    egui::Image::new(&texture).paint_at(ui, rect);
}


/////////////////////////////

pub fn wndw_main_empty(ui: &egui::Context) -> ()
{
    egui::CentralPanel::default().show(ui, |ui| {
        let window_area = egui::Rect{min:ui.next_widget_position(), 
            max:ui.next_widget_position() + ui.available_size()};

        ui.put(window_area, egui::Label::new("no images opened".to_string())); 
    });
}

pub fn wndw_main(ui: &egui::Context, img_data: &mut Data, data_shared: &mut Shared) -> ()
{
    let img = &mut img_data.folders[data_shared.main_img.folder].images[data_shared.main_img.image];

    egui::CentralPanel::default().show(ui, |ui| {

        let offset = ui.next_widget_position();
        let window_area = egui::Rect{min:offset, max:offset + ui.available_size()};

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
                let texture = get_frame(ui, img, data_shared);
                let (transform, interacted) = calc_transform(ui, img);
                show_img_area(ui, texture, transform);
                if interacted {img.transform = Some(transform);}
            }

            Status::Error => 
            { 
                let msg = "error loading ".to_string() + &img.file;
                ui.put(window_area, egui::Label::new(&msg));
            }
        }
    });
}