use std::{process, io::{self, Read}};
use raw_keys::RawMode;
use crate::cli::content::{get_content_of_current_dir, update_content};
use crate::cli::input::{handle_input, handle_sequence};

pub struct Dir{
    parent_path: String,
    path: String,
    content: Vec<String>,
    index: i32,
    length: usize,
}

impl Dir {
    pub fn new() -> Self {
        Dir { parent_path: String::new(), path:String::from("/home/wsl/Rust"), content:Vec::new(), index:0, length:0 }
    }
    pub fn reset(&mut self) {
        self.index = 0;
        &self.content.clear();
        self.length = 0;
    }
    pub fn content(&self) -> &Vec<String> {
        &self.content
    }
    pub fn get_content(&self, index: i32) -> &String {
        &self.content[index as usize]
    }
    pub fn clear_content(&mut self) {
        let _ = &self.content.clear();
    }
    pub fn push_content(&mut self, value: String) {
        let _ = &self.content.push(value);
    }
    pub fn index(&self) -> &i32 {
        &self.index
    }
    pub fn change_index(&mut self, value: i32) {
        self.index += value;
    }
    pub fn length(&self) -> &usize {
        &self.length
    }
    pub fn change_length(&mut self, new_length: usize) {
        self.length = new_length;
    }
    pub fn parent(&self) -> &str {
        &self.parent_path
    }
    pub fn change_parent(&mut self, parent: String) {
        let _ = self.parent_path = parent;
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn change_path(&mut self, new_path: String) {
       let _ = self.path = new_path;
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
            if char_equivelant == ' ' {
                continue;
            }
            let action = handle_input(char_equivelant, &dir.index(), &dir.length());
            update_content(&mut dir, action);
            continue;
        }
        let action = handle_input(char, &dir.index(), &dir.length());
        update_content(&mut dir, action);
    }
}

