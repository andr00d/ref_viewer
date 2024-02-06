use eframe::egui::{Ui, ColorImage, TextureHandle};
use std::thread::{self, JoinHandle};
use image::imageops::FilterType;

#[derive(PartialEq)]
pub enum Status 
{
    Unloaded,
    Loading,
    Loaded,
    Error,
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
pub struct Index
{
    pub folder: usize,
    pub image: usize, 
}

pub struct Image
{
    pub file: String,
    pub artists: Vec<String>, 
    pub size: String, 
    pub links: Vec<String>, 
    pub tags: Vec<String>,

    // thumbnail
    pub thumb_texture: Option<TextureHandle>,
    thumb_thread: Option<JoinHandle<Result<ColorImage, String>>>,
    thumb_state: Status,

    // full image
    pub full_texture: Option<TextureHandle>,
    pub full_scale: Option<f32>,
    full_thread: Option<JoinHandle<Result<ColorImage, String>>>,
    full_state: Status,
}


impl Image 
{
    pub fn new(
        file: String,
        artists: Vec<String>, 
        size: String, 
        links: Vec<String>, 
        tags: Vec<String>,
        ) -> Image 
    {
        Image{
        file : file, 
        artists : artists, 
        size: size, 
        links: links, 
        tags: tags,

        thumb_texture: None,
        thumb_thread: None,
        thumb_state: Status::Unloaded,

        full_texture: None,
        full_scale: None, 
        full_thread: None,
        full_state: Status::Unloaded,
        }
    }

    fn create_thr(path: String, thumbnail: bool) -> JoinHandle<Result<ColorImage, String>>
    {
        thread::spawn(move || -> Result<ColorImage, String>
        {
            let input = match image::io::Reader::open(path.clone())
            {
                Ok(x) => x,
                Err(_x) => return Err(format!("{} does not exist.", path)),
            };

            let decoded = match input.decode()
            {
                Ok(x) => x,
                Err(_x) => return Err(format!("Error when decoding {}.", path)),
            };

            let image = match thumbnail
            {
                true => decoded.resize(100, 100, FilterType::Nearest),
                false => decoded,
            };

            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();

            Ok(egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice(),))
        })
    }

    ////////////////////////
    // (de)loading images //
    ////////////////////////

    pub fn load_thumb(&mut self) -> bool
    {
        if self.thumb_state == Status::Loading {println!("tried to load thumb twice");}

        self.thumb_thread = Some(Self::create_thr(self.file.clone(), true));
        self.thumb_state = Status::Loading;
        return true
    }

    pub fn load_full(&mut self) -> bool
    {
        if self.full_state == Status::Loading {println!("tried to load image twice");}

        self.full_thread = Some(Self::create_thr(self.file.clone(), false));
        self.full_state = Status::Loading;
        return true
    }

    pub fn clear_full(&mut self) -> bool
    {
        if self.full_state != Status::Loaded {return false;}

        self.full_texture = None;
        self.full_state = Status::Unloaded;
        return true;
    }

    fn poll_load(&mut self, ui: &mut Ui, thumbnail: bool) -> ()
    {
        let thread = match thumbnail
        {
            true => &mut self.thumb_thread,
            false => &mut self.full_thread,
        };

        let state = match thumbnail
        {
            true => &mut self.thumb_state,
            false => &mut self.full_state,
        };

        if thread.as_ref().is_none()
        {
            *state = Status::Unloaded;
            match thumbnail
            {
                true => println!("tried to poll thumbnail before creating it ({})", self.file),
                false => println!("tried to poll image before creating it ({})", self.file),
            };
            return;
        }

        if !thread.as_ref().unwrap().is_finished()
        {
            return;
        }

        let thread_result = match thread.take().unwrap().join()
        {
            Ok(x) => x,
            Err(_x) => 
            {
                println!("Thread error for {}", self.file);
                *state = Status::Error;
                return
            }
        };


        let result = match thread_result
        {
            Ok(x) => x,
            Err(_x) => 
            {
                println!("image loading/scaling error for {}", self.file);
                *state = Status::Error;
                return
            }
        };

        let texture = match thumbnail
        {
            true => &mut self.thumb_texture,
            false => &mut self.full_texture,
        };

        *texture = Some(ui.ctx().load_texture(
                        self.file.clone(),
                        result,
                        Default::default(), 
                    ));

        *state = Status::Loaded;
    }

    pub fn poll_thumb(&mut self, ui: &mut Ui) -> ()
    {
        Self::poll_load(self, ui, true);
    }

    pub fn poll_full(&mut self, ui: &mut Ui) -> ()
    {
        Self::poll_load(self, ui, false);
    }

    ////////////////
    // image info //
    ////////////////

    pub fn thumb_state(&self) -> &Status
    {
        return &self.thumb_state;
    }

    pub fn full_state(&self) -> &Status
    {
        return &self.full_state;
    }

    pub fn add_tag(&mut self, tag: &String) -> ()
    {
        if !self.tags.contains(&tag.to_lowercase())
        {
            self.tags.push(tag.to_lowercase().clone());
            self.tags.sort();
        }
    }

    pub fn remove_tag(&mut self, tag: &String) -> bool
    {
        match self.tags.iter().position(|x| x == tag)
        {
            Some(index) => self.tags.remove(index),
            None => return false,
        };

        return true
    }  

    pub fn add_link(&mut self, link: &String) -> ()
    {
        println!("{}", link);
        if !self.links.contains(&link)
        {
            self.links.push(link.clone());
            self.links.sort();
        }
    }

    pub fn remove_link(&mut self, link: &String) -> bool
    {
        match self.links.iter().position(|x| x == link)
        {
            Some(index) => self.links.remove(index),
            None => return false,
        };

        return true
    }  

    pub fn add_artist(&mut self, artist: &String) -> ()
    {
        println!("{}", artist);
        if !self.artists.contains(&artist)
        {
            self.artists.push(artist.clone());
            self.artists.sort();
        }
    }

    pub fn remove_artist(&mut self, artist: &String) -> bool
    {
        match self.artists.iter().position(|x| x == artist)
        {
            Some(index) => self.artists.remove(index),
            None => return false,
        };

        return true
    }  
}
