use std::time::Instant;

use crate::shared::{Shared, Gallery};
use crate::data::image::Index;

impl Shared 
{
    pub fn new(imagelist: Vec<Vec<Index>>, index: Index) -> Shared
    {
        let mut count = 0;

        for folder in &imagelist
        {
            count += folder.len();
        }

        Shared{main_img: index,
            active_input: None,
            gallery_type: Gallery::LeftBar,
            last_update: Instant::now(),
            frame_index: 0,
            key_event: None,
            search: "".to_string(),
            results: imagelist,
            results_len: count}
    }

    pub fn next_result(&self) -> Option<Index>
    {
        let mut f = self.main_img.folder;
        let pos = self.results[f].iter().position(|n| *n == self.main_img);
        if pos.is_none() {return None;}
        
        
        if pos.unwrap() == self.results[f].len() - 1
        {
            loop
            {
                f = (f + 1) % self.results.len();
                if self.results[f].len() == 0 {continue;}
                return Some(self.results[f][0].clone());
            }
        }

        return Some(self.results[f][pos.unwrap()+1].clone());
    }

    pub fn prev_result(&self) -> Option<Index>
    {
        let mut f = self.main_img.folder;
        let pos = self.results[f].iter().position(|n| *n == self.main_img);
        if pos.is_none() {return None;}
        
        
        if pos.unwrap() == 0
        {
            loop
            {
                if f == 0 {f = self.results.len() - 1;}
                else {f = f - 1;}
                
                let len = self.results[f].len();
                if len == 0 {continue;}
                return Some(self.results[f][len-1].clone());
            }
        }

        return Some(self.results[f][pos.unwrap()-1].clone());
    }

    pub fn set_results(&mut self, imagelist: Vec<Vec<Index>>) -> ()
    {
        self.results = imagelist;
        let mut count = 0;

        for folder in &self.results
        {
            count += folder.len();
        }

        self.results_len = count;
    }

    pub fn get_results(&self) -> &Vec<Vec<Index>>
    {
        return &self.results;
    }

    pub fn get_result_size(&self) -> usize
    {
        return self.results_len;
    }
}