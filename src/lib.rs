// Lib file for Workjournal

use std::path::PathBuf;
use chrono::prelude::*;


pub fn wip() {
    get_file_handle();
}


pub struct Config {
    active_job: u32,
    logging_folder: PathBuf,
    file_extension: Option<String>,
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

fn get_file_handle() {
    let logging_folder = PathBuf::from("/home/truepenny/logs_from_work");
    let d_and_t = chrono::Local::now();
    let date = d_and_t.date_naive();
    let mut formatted = date.format("%Y-%m-%d").to_string();
    formatted.push_str("-DL.txt");
    let mut fullpath = logging_folder;
    fullpath.push(formatted.clone());
    println!("file: {}", fullpath.display());
}
