use terminal_size::{terminal_size, Height, Width};

pub struct Global {
    pub start: usize,
    pub window_size: usize,
    pub hidden_files: bool,
}

impl Global {
    pub fn new() -> Self {
        Global {
            start: 0,
            window_size: get_terminal_size(),
            hidden_files: false,
        }
    }
    pub fn change_start(&mut self, value: usize){
        self.start = value;
    }
    pub fn switch_hf(&mut self) {
        self.hidden_files = !self.hidden_files;
    }
}

pub struct Element {
    pub name: String,
    pub element_type: Type,
    pub selected: bool,
}

impl Element {
    pub fn new(name: String, element_type: Type) -> Self {
        Element {
            name: name,
            selected: false,
            element_type: element_type,
        }
    }
    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
}

pub enum Type {
    Return,
    File,
    Directory,
    HiddenFile,
    Other,
}

pub struct Dir {
    pub parent_path: String,
    pub path: String,
    pub content: Vec<Element>,
    pub index: i32,
    pub length: usize,
}

impl Dir {
    pub fn new() -> Self {
        Dir { 
            parent_path: String::new(), 
            path: String::from("/home/wsl/Rust"), 
            content: Vec::new(), 
            index: 0, 
            length: 0,
        }
    }
    pub fn reset(&mut self) {
        let _ = &self.content.clear();
        self.length = 0;
    }
    pub fn get_content(&self, index: i32) -> &Element {
        &self.content[index as usize]
    }
    pub fn get_content_mut(&mut self, index: i32) -> &mut Element {
        &mut self.content[index as usize]
    }
    pub fn push_content(&mut self, value: Element) {
        let _ = &self.content.push(value);
    }
    pub fn change_index(&mut self, value: i32) {
        self.index += value;
    }
    pub fn change_length(&mut self, new_length: usize) {
        self.length = new_length;
    }
    pub fn change_parent(&mut self, parent: String) {
        self.parent_path = parent;
    }
    pub fn change_path(&mut self, new_path: String) {
       self.path = new_path;
    }

}

pub enum Action{
    Empty,
    Up,
    Down,
    Enter,
    ShowHiddenFiles,
    Select,
    AddFile,
    AddDirectory,
    Delete,
}

fn get_terminal_size() -> usize{
    let size = terminal_size();
    if let Some((Width(_w), Height(h))) = size {
        println!("{h}");
        //minus the length of the whitespaces and absolute path
        (h - 5) as usize
    }
    else {
        //defualt
        20
    }
}

