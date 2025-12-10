use std::{process, io::{self, Read}};
use raw_keys::RawMode;
use terminal_size::{terminal_size, Height, Width};
use crate::cli::content::{get_content_of_current_dir, update_content};
use crate::cli::input::{handle_input, handle_sequence};

pub struct Dir{
    pub parent_path: String,
    pub path: String,
    pub content: Vec<String>,
    pub index: i32,
    pub length: usize,
    pub start: usize,
    pub window_size: usize,
}

impl Dir {
    pub fn new() -> Self {
        Dir { 
            parent_path: String::new(), 
            path:String::from("/home/wsl/Rust"), 
            content:Vec::new(), 
            index:0, 
            length:0,
            start: 0,
            window_size: get_terminal_size(),
        }
    }
    pub fn reset(&mut self) {
        self.index = 0;
        let _ = &self.content.clear();
        self.length = 0;
        self.start = 0;
    }
    pub fn get_content(&self, index: i32) -> &String {
        &self.content[index as usize]
    }
    pub fn push_content(&mut self, value: String) {
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
    pub fn change_start(&mut self, value: usize){
        self.start = value;
    }
}

#[derive(PartialEq)]
pub enum Action{
    NONE,
    UP,
    DOWN,
    ENTER,
}

pub fn run_cli(){

    let mut dir = Dir::new();
    
    let _raw_mode = RawMode::new().unwrap();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut c = [0u8; 1];

    if let Err(e) = get_content_of_current_dir(&mut dir){
        eprintln!("{e}");
        process::exit(1);
    };

    update_content(&mut dir, Action::NONE);

    while handle.read(&mut c).unwrap() == 1 && c[0] != b'q' {
        //println!("{}", c[0] as char);
        let char = c[0] as char;

        if char == '\x1b' {
            let mut seq1 = [0u8; 1];
            if handle.read(&mut seq1).unwrap() != 1 {
                continue;
            }
            if seq1[0] != b'[' {
                continue;
            }
            let mut seq2 = [0u8; 1];
            if handle.read(&mut seq2).unwrap() != 1 {
                continue;
            }

            let char2 = seq2[0] as char;
            let char_equivelant = handle_sequence(char2);

            match char_equivelant {
                Some(c) => {
                    let action = handle_input(c, dir.index, dir.length);
                    update_content(&mut dir, action);
                    continue;
                },
                None => continue,
            };
        }
        let action = handle_input(char, dir.index, dir.length);
        update_content(&mut dir, action);
    }
}

fn get_terminal_size() -> usize{
    let size = terminal_size();
    if let Some((Width(_w), Height(h))) = size {
        println!("{h}");
        (h - 4) as usize
    }
    else {
        //defualt
        20
    }
}

