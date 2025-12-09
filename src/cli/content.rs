use std::{path::Path, error::Error, fs};

pub fn get_content_of_current_dir(global_path: &str, content: &mut Vec<String>) -> Result<(), Box<dyn Error>>{
    let path = Path::new(global_path);
    content.clear();
    content.push(String::from("(back)"));
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

pub fn show_content(content: &Vec<String>, index: i32) {
    print!("\x1B[2J\x1B[1;1H");
    for (i, element) in content.iter().enumerate() {
        if i as i32 == index {
            println!("{element} <-");
            continue;
        }
        println!("{element}");
    }
}
