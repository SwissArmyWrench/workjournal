// Lib file for Workjournal

use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use regex::Regex;
use grep::matcher::Matcher;

pub fn wip() {
    /* let config: Config = Config {
        active_job: 49390,
        logging_folder: PathBuf::from("/home/truepenny/logs_from_work"),
        file_extension: Some(String::from(".txt")),
    }; */

    let config = Config::load().unwrap();
    // let path_to_file = PathBuf::from("/home/truepenny/logs_from_work/2024-10-8-xL.txt");
    let file = File::open("/home/truepenny/logs_from_work/2024-10-08-DL.txt").unwrap();
    println!("{:?}", grep_as_lines(file, "#49882".to_string()));
    // change_job_yaml(49999);
    
    /*
    let mut file = config.get_today_handle();
    std::fs::write(format!("The active job is {}", config.active_job).as_bytes(), file);
    file.write(format!("The active job is {}\n", config.active_job).as_bytes());
    */
    // println!("Active job: {}", config.active_job);
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    active_job: u32,
    logging_folder: PathBuf,
    file_extension: Option<String>,
}

impl Config {
    fn get_today_handle(&self) -> std::fs::File {
        // let logging_folder = PathBuf::from("/home/truepenny/logs_from_work");
        let d_and_t = chrono::Local::now();
        let date = d_and_t.date_naive();
        let mut formatted: String = date.format("%Y-%m-%d").to_string();
        formatted.push_str("-DL");
        match &self.file_extension {
            Some(extension) => {formatted = format!("{}{}", formatted, extension)} // .push_str(extension), // Add file extension if specified
            None => {}                                        // do nothing
        }
        let mut fullpath = self.logging_folder.clone();
        fullpath.push(formatted.clone());
        // println!("{}", fullpath.display());
        match OpenOptions::new().append(true).open(fullpath.clone()) {
            Ok(file) => file,
            Err(_) => OpenOptions::new()
                .create(true)
                .append(true)
                .open(fullpath)
                .unwrap(),
        }
    }

    pub fn load() -> Result<Config, serde_yaml::Error> {
        let dirs = ProjectDirs::from("com", "SwissArmyWrench", "Workjournal").unwrap(); // SAFE
        let mut config_path = dirs.config_dir().to_owned();
        config_path.push("config.yaml");
        let reader = File::open(config_path).expect("The system was unable to open the config file");
        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(reader);
        config // needed to define a result to turn in order to give the type hint above
        
    }
}

pub struct Command {
    args: Vec<String>,
    intent: Intent,
    config: Config
}

impl Command {
    pub fn run(self) {
        // println!("Running...");
        match self.intent {
            Intent::MakeNote(note) => {
                let time = chrono::Local::now().time().format("%H:%M").to_string();
                let _ = &self.config.get_today_handle().write(format!("{time} #{0} {note}\n", self.config.active_job.to_string()).as_bytes());
            }
            Intent::ChangeActive(job_number) => { change_job_yaml(job_number); }
            _ => {}
        }
    }

    pub fn new(args: Vec<String>, intent: Intent, config: Config) -> Command {
        Command { args: args,
            intent: intent,
            config: config }
    }
}

pub enum Intent {
    ChangeActive(u32),
    MakeNote(String),
    NoCmd,
}


fn change_job_yaml(newjob: u32) {
    // Get the yaml config as a string
    let dirs = ProjectDirs::from("com", "SwissArmyWrench", "Workjournal").unwrap(); // SAFE
    let mut config_path = dirs.config_dir().to_owned();
    config_path.push("config.yaml");
    let config_string = std::fs::read_to_string(&config_path).expect("Unable to open config file to change jobs");

     
    // Regex to match on the key:value pair in the YAML
    let regex = Regex::new(r"active_job: (?<number>\d{5})").unwrap();
    let after = regex.replace_all(&config_string, format!("active_job: {}", newjob.to_string()));
    // println!("{}", after);
    match std::fs::remove_file(&config_path) {
        Ok(_) => {}
        Err(_) => { println!("Unable to remove config file in order to re-write"); }
    }
    let mut config_file = File::create(&config_path).expect("Unable to re-write new config");
    let _ = config_file.write(after.as_bytes());

    


}

fn grep_as_lines(path: File, query: String) -> Vec<String> {
    // Vec to store the matches to the query
    let mut matches = Vec::<String>::new();

    // Build searcher
    let mut builder = grep::searcher::SearcherBuilder::new();
    builder.line_number(true);
    let mut searcher = builder.build();

    
    // Build matcher
    // TODO: build code to convert the query to the regex matcher
    let matcher = grep::regex::RegexMatcher::new(&query).unwrap();
    
    // Build UTF8 sink
    let sink = grep::searcher::sinks::UTF8(| line_number, line | {
        match matcher.find(line.as_bytes()).unwrap() {
            Some(_) => { matches.push(line.to_string()); },
            None => {}
        }
        Ok(true)
    });

    let out = searcher.search_file(&matcher, &path, sink);
    println!("{:?}", out);
    // return matches after running the search
    matches
}
