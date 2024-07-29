use std::path::Path;
use std::thread::{self, JoinHandle};
use eframe::egui::{Ui, ColorImage, TextureHandle};
use image::imageops::FilterType;
use image::codecs::gif::GifDecoder;
use image::codecs::webp::WebPDecoder;
use image::DynamicImage;
use image::AnimationDecoder;

/////////////////////////

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

pub struct FrameData {
    pub image: ColorImage,
    pub delay: u32,
}

pub struct TextureData {
    pub image: TextureHandle,
    pub delay: u32,
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

    // full view
    pub full_texture: Vec<TextureData>,
    pub full_scale: Option<f32>,
    full_thread: Option<JoinHandle<Result<Vec<FrameData>, String>>>,
    full_state: Status,
}

/////////////////////////

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

        full_texture: Vec::new(),
        full_scale: None, 
        full_thread: None,
        full_state: Status::Unloaded,
        }
    }

    fn create_thr_thumb(path: String) -> JoinHandle<Result<ColorImage, String>>
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
 
            let mut background = image::ImageBuffer::from_pixel(100, 100, image::Rgba([0,0,0,0]));
            let mut image = decoded.resize(100, 100, FilterType::Nearest);
            
            let x_offset = ((100 - image.width()) / 2) as i64;
            let y_offset = ((100 - image.height()) / 2) as i64;
            image::imageops::overlay(&mut background, &mut image, x_offset, y_offset);
            
            let size = [background.width() as _, background.height() as _];
            // let image_buffer = background.to_rgba8();
            let pixels = background.as_flat_samples();

            Ok(egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice(),))
        })
    }

    fn create_thr_full(path: String) -> JoinHandle<Result<Vec<FrameData>, String>>
    {
        thread::spawn(move || -> Result<Vec<FrameData>, String>
        {
            let file = match image::io::Reader::open(path.clone())
            {
                Ok(x) => x,
                Err(_x) => return Err(format!("{} does not exist.", path)),
            };

            let mut images = Vec::<FrameData>::new();
            // handle webp and gif animations besides normal images by dumping everything into a vector
            match Path::new(&path).extension().unwrap().to_str().unwrap()
            {
                "webp" => 
                {
                    // TPDP: check further in specifics of has_animation
                    let decoder = match WebPDecoder::new(file.into_inner())
                    {
                        Ok(x) => x,
                        Err(_x) => return Err(format!("malformed webp file: {}.", path)),
                    };

                    // into_frames doesn't work for webp images, only webp animations.
                    if !decoder.has_animation()
                    {
                        let frame = DynamicImage::from_decoder(decoder).unwrap().to_rgba8();
                        let size = [frame.width() as _, frame.height() as _];
                        let img = egui::ColorImage::from_rgba_unmultiplied(size, &frame);
                        images.push(FrameData{image: img, delay: 0});
                    }
                    else
                    {
                        let frames = match decoder.into_frames().collect_frames()
                        {
                            Ok(x) => x,
                            Err(_x) => return Err(format!("malformed webp file: {}.", path)),
                        };
                            
                        for frame in frames
                        {
                            let size = [frame.buffer().width() as _, frame.buffer().height() as _];
                            let img = egui::ColorImage::from_rgba_unmultiplied(size, frame.buffer());
                            let (numerator, denominator) = frame.delay().numer_denom_ms();
                            let delay = numerator / denominator; 
                            images.push(FrameData{image: img, delay: delay});
                        };
                    }
                    
                },
                "gif" => 
                {
                    let decoder = match GifDecoder::new(file.into_inner())
                    {
                        Ok(x) => x,
                        Err(_x) => return Err(format!("malformed gif file: {}.", path)),
                    };

                    let frames = match decoder.into_frames().collect_frames()
                    {
                        Ok(x) => x,
                        Err(_x) => return Err(format!("malformed gif file: {}.", path)),
                    };

                    for frame in frames
                    {
                        let size = [frame.buffer().width() as _, frame.buffer().height() as _];
                        let img = egui::ColorImage::from_rgba_unmultiplied(size, frame.buffer());
                        let (numerator, denominator) = frame.delay().numer_denom_ms();
                        let delay = numerator / denominator;
                        images.push(FrameData{image: img, delay: delay});
                    };
                    
                },
                _ => 
                {
                    match file.decode()
                    {
                        Ok(x) =>
                        {
                            let size = [x.width() as _, x.height() as _];
                            let image_buffer = x.to_rgba8();
                            let pixels = image_buffer.as_flat_samples();
                            let image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                            images.push(FrameData{image: image, delay: 0});
                        },
                        Err(_x) => return Err(format!("malformed image file: {}.", path)),
                    }
                }, 
            }

            Ok(images)
        })
    }


    ////////////////////////
    // (de)loading images //
    ////////////////////////

    pub fn load_thumb(&mut self) -> bool
    {
        if self.thumb_state == Status::Loading {println!("tried to load thumb twice");}

        self.thumb_thread = Some(Self::create_thr_thumb(self.file.clone()));
        self.thumb_state = Status::Loading;
        return true
    }

    pub fn load_full(&mut self) -> bool
    {
        if self.full_state == Status::Loading {println!("tried to load image twice");}

        self.full_thread = Some(Self::create_thr_full(self.file.clone()));
        self.full_state = Status::Loading;
        return true
    }

    pub fn clear_full(&mut self) -> bool
    {
        if self.full_state != Status::Loaded {return false;}

        self.full_texture = Vec::new();
        self.full_state = Status::Unloaded;
        return true;
    }

    pub fn poll_thumb(&mut self, ui: &mut Ui) -> ()
    {
        if self.thumb_thread.as_ref().is_none()
        {
            self.thumb_state = Status::Unloaded;
            println!("tried to poll thumbnail before creating it ({})", self.file);
            return;
        }

        if !self.thumb_thread.as_ref().unwrap().is_finished() {return;}

        let thread_result = match self.thumb_thread.take().unwrap().join()
        {
            Ok(x) => x,
            Err(x) => 
            {
                println!("Thread error when loading thumb for {}", self.file);
                println!("details: {:?}", x);
                self.thumb_state = Status::Error;
                return;
            }
        };

        let result = match thread_result
        {
            Ok(x) => x,
            Err(x) => 
            {
                println!("image loading/scaling error for {}", self.file);
                println!("details: {:?}", x);
                self.thumb_state = Status::Error;
                return
            }
        };
        
        let texture = ui.ctx().load_texture(self.file.clone(), result, Default::default());
        self.thumb_texture = Some(texture);
        self.thumb_state = Status::Loaded;
    }

    pub fn poll_full(&mut self, ui: &mut Ui) -> ()
    {
        if self.full_thread.as_ref().is_none()
        {
            self.full_state = Status::Unloaded;
            println!("tried to poll image before creating it ({})", self.file);
            return;
        }

        if !self.full_thread.as_ref().unwrap().is_finished() {return;}

        let thread_result = match self.full_thread.take().unwrap().join()
        {
            Ok(x) => x,
            Err(_x) => 
            {
                println!("Thread error when loading image for {}", self.file);
                self.full_state = Status::Error;
                return
            }
        };

        let result = match thread_result
        {
            Ok(x) => x,
            Err(_x) => 
            {
                println!("image loading error for {}", self.file);
                self.full_state = Status::Error;
                return
            }
        };

        let mut buffer = Vec::new();

        for frame in result
        {
            let texture = ui.ctx().load_texture(self.file.clone(), frame.image, Default::default());
            buffer.push(TextureData{image: texture, delay: frame.delay});
        }

        self.full_texture = buffer;
        self.full_state = Status::Loaded;
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
