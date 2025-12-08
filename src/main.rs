use std::{process, fs, path::Path, error::Error, io::{self, Read}};
use raw_keys::{RawMode};

fn main() {
    run();
}

fn run(){
    let global_path = String::from(".");
    let mut content: Vec<String> = Vec::new();
    if let Err(e) = get_content_of_current_dir(&global_path, &mut content){
        eprintln!("{e}");
        process::exit(1);
    }

    show_content(&content);

    let raw_mode = RawMode::new().unwrap();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut c = [0u8; 1];

    while handle.read(&mut c).unwrap() == 1 && c[0] != b'q' {
        println!("{}", c[0] as char);
    }

}

fn get_content_of_current_dir(global_path: &str, content: &mut Vec<String>) -> Result<(), Box<dyn Error>>{
    let path = Path::new(global_path);
    content.clear();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_str().unwrap().to_string();
                content.push(file_name);
            }
        }
    }
    Ok(())
}

fn show_content(content: &Vec<String>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("<-");
    for element in content {
        println!("{element}");
    }
}
