use std::collections::HashMap;
use std::time::Instant;
use regex::Regex;

use crate::shared::{Shared, Gallery};
use crate::data::image::Index;
use crate::data::Data;

impl Shared 
{
    pub fn new(imagelist: Vec<Vec<Index>>, index: Index) -> Shared
    {
        let mut count = 0;
        for folder in &imagelist { count += folder.len(); }

        let mut selected = Vec::new();
        selected.push(index.clone());

        //order: [artists, links, tags]
        let tags_array: [Vec<(String, usize)>; 3] = Default::default();

        Shared{main_img: index,
            active_input: None,
            gallery_type: Gallery::LeftBar,
            last_update: Instant::now(),
            frame_index: 0,
            key_event: None,
            search: "".to_string(),
            selected: selected,
            selected_tags: tags_array,
            results: imagelist,
            results_len: count,}
    }

    //////////////////////////

    pub fn next_result(&self, index: &Index) -> Option<Index>
    {
        let mut f = index.folder;
        let pos = self.results[f].iter().position(|n| n == index);
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

    pub fn prev_result(&self, index: &Index) -> Option<Index>
    {
        let mut f = index.folder;
        let pos = self.results[f].iter().position(|n| n == index);
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

    //////////////////////////

    pub fn update_tags(&mut self, img_data: &Data)
    {
        for tags in &mut self.selected_tags {tags.clear();}
        let mut artistcounts: HashMap<String, usize> = HashMap::new();
        let mut linkcounts: HashMap<String, usize> = HashMap::new();
        let mut tagcounts: HashMap<String, usize> = HashMap::new();
        
        for index in &self.selected
        {
            let img = &img_data.folders[index.folder].images[index.image];
            
            for tag in &img.artists {*artistcounts.entry(tag.to_string()).or_insert(0) += 1;}
            for tag in &img.links   {*linkcounts.entry(tag.to_string()).or_insert(0) += 1;}
            for tag in &img.tags    {*tagcounts.entry(tag.to_string()).or_insert(0) += 1;}
        }
        
        self.selected_tags[0] = artistcounts.into_iter().collect();
        self.selected_tags[1] = linkcounts.into_iter().collect();
        self.selected_tags[2] = tagcounts.into_iter().collect();

        for i in 0..self.selected_tags.len()
        {
            self.selected_tags[i].sort_by(|a, b| {
                if a.1 != b.1  {return b.1.cmp(&a.1);}
                else {return b.0.cmp(&a.0)};
            });
        }
    }

    pub fn add_selected(&mut self, img_data: &mut Data, index: &Index) -> ()
    {
        self.selected.push(index.clone());
        self.update_tags(img_data);
    }

    pub fn set_selected(&mut self, img_data: &Data, a: &Index, b: &Index) -> ()
    {
        let a_pos = self.results[a.folder].iter().position(|n| n == a);
        let b_pos = self.results[b.folder].iter().position(|n| n == b);
        if a_pos.is_none() && b_pos.is_none() {return;}

        let (start, end); 
        if a.folder < b.folder || (a.folder == b.folder && a.image < b.image)
        {
            start = a.clone();
            end = b.clone();
        }
        else
        {
            start = b.clone();
            end = a.clone();
        } 

        self.selected.clear();
        self.selected.push(start.clone());

        while self.selected.last().unwrap() != &end
        {
            let index = self.next_result(&self.selected.last().unwrap());
            self.selected.push(index.unwrap());
        }

        self.update_tags(img_data);
    }

    pub fn get_selected(&self) -> &Vec<Index>
    {
        return &self.selected;
    }

    pub fn get_selected_tags(&self) -> &[Vec<(String, usize)>; 3]
    {
        return &self.selected_tags;
    }

    //////////////////////////

    pub fn update_search(&mut self, img_data: &Data) -> ()
    {
        let mut tags: Vec<String> = self.search.split_whitespace().map(str::to_string).collect();
        let mut itags = tags.clone();

        tags.retain(|x| !x.starts_with("-"));
        itags.retain(|x| x.starts_with("-"));
        for part in &mut itags{part.remove(0);}


        self.results = img_data.build_vector(tags, itags);

        let mut count = 0;
        for folder in &self.results {count += folder.len();}
        self.results_len = count;

        if !self.results[self.main_img.folder].contains(&self.main_img) && self.results.len() > 0
        {
            let mut index = Index{folder: 0, image: 0};
            for folder in &self.results 
            { 
                if folder.len() > 0 {index = folder[0].clone(); break;} 
            }
            self.set_selected(img_data, &index, &index);
            self.main_img = index;
        }
    }

    pub fn rem_from_search(&mut self, img_data: &Data, tag: &String)
    {
        self.search = str::replace(&self.search, tag, "");

        let re = Regex::new(r"\s\s+").unwrap();
        self.search = re.replace_all(&self.search, " ").to_string();

        self.update_search(img_data);
    }

    pub fn add_to_search(&mut self, img_data: &Data, tag: &String)
    {
        if self.search.contains(tag) {return;}
        self.search += &(" ".to_owned() + tag);

        let re = Regex::new(r"\s\s+").unwrap();
        self.search = re.replace_all(&self.search, " ").to_string();

        self.update_search(img_data);
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