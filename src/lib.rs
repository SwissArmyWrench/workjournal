// Lib file for Workjournal

use std::path::PathBuf;
// use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

pub fn wip() {
    let config: Config = Config { 
        active_job: 49390,
        logging_folder: PathBuf::from("/home/truepenny/logs_from_work"),
        file_extension: Some(".txt")
    };
    let mut file = config.get_today_handle();
    // std::fs::write(format!("The active job is {}", config.active_job).as_bytes(), file);
    file.write(format!("The active job is {}\n", config.active_job).as_bytes());
}


pub struct Config {
    active_job: u32,
    logging_folder: PathBuf,
    file_extension: Option<&'static str>,
}

impl Config {
fn get_today_handle(&self) -> std::fs::File {
    // let logging_folder = PathBuf::from("/home/truepenny/logs_from_work");
    let d_and_t = chrono::Local::now();
    let date = d_and_t.date_naive();
    let mut formatted: String = date.format("%Y-%m-%d").to_string();
    formatted.push_str("-DL");
    match self.file_extension {
        Some(extension) => {formatted.push_str(extension)}, // Add file extension if specified
        None => {} // do nothing
    }
    let mut fullpath = self.logging_folder.clone();
    fullpath.push(formatted.clone());
    println!("{}", fullpath.display());
    match OpenOptions::new().append(true).open(fullpath.clone()) {
        Ok(file) => { file },
        Err(_) => {OpenOptions::new().create(true).append(true).open(fullpath).unwrap()}
    }
}

    fn import_config() /* -> Config */ {
        
    }
}

pub struct Command {
    args: Vec<String>,
    // config:
} 

impl Command {
    fn run(self) {
        println!("Running...");
    }
}

enum Intent {
    ChangeActive(u32),
    MakeNote(String),

}


