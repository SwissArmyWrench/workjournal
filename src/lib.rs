// Lib file for Workjournal

use directories::ProjectDirs;
use grep::matcher::Matcher;
use natural_sort_rs::{Natural, NaturalSort};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use clap::Subcommand;

/*pub fn wip() {
    // WIP
    let config = Config::load(); 
    let command = Command { args: Vec::new(), config: config.expect("couldn't load config"), intent: Intent::PrintNotes(49882) };
    command.run();
}*/

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
            Some(extension) => formatted = format!("{}{}", formatted, extension), // .push_str(extension), // Add file extension if specified
            None => {}                                                            // do nothing
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
                .unwrap()
        }
    }

    pub fn load() -> Result<Config, serde_yaml::Error> {
        let dirs = ProjectDirs::from("com", "SwissArmyWrench", "Workjournal").unwrap(); // SAFE
        let mut config_path = dirs.config_dir().to_owned();
        config_path.push("config.yaml");
        let reader =
            File::open(config_path).expect("The system was unable to open the config file");
        let config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(reader);
        config // needed to define a result to turn in order to give the type hint above
    }
}

pub struct Command {
    args: Vec<String>,
    intent: Subcommands,
    config: Config,
}

impl Command {
    pub fn run(self) {
        // println!("Running...");
        match self.intent {
            Subcommands::Mknote {note, job}  => {
                let job = match job {
                    Some(jobnumber) => jobnumber,
                    None => self.config.active_job
                };
                let time = chrono::Local::now().time().format("%H:%M").to_string();
                let _ = &self.config.get_today_handle().write(
                    format!("{time} #{0} {note}\n", job.to_string()).as_bytes(),
                );
            }
            /*Intent::MakeNoteOnJob(note, number) => {
                let time = chrono::Local::now().time().format("%H:%M").to_string();
                let _ = &self.config.get_today_handle().write(
                    format!("{time} #{0} {note}\n", number.to_string()).as_bytes(),
                );

            }*/
            Subcommands::Chactive {jobnumber} => {
                change_job_yaml(jobnumber);
            }
            Subcommands::Print {jobnumber} => {
                let pathlist = get_paths(&self.config.logging_folder);
                for path in pathlist {
                    let notes =
                        grep_as_lines(PathBuf::from(&path), format!("#{}", jobnumber.to_string()));
                    if !notes.is_empty() {
                        println!("Job {jobnumber} in {}", PathBuf::from(&path).file_stem().unwrap().to_string_lossy().to_string());
                    }
                    notes.iter().for_each(|note| println!("{}\n", note));
                }
            }
            Subcommands::Active => {println!("Job {} is currently active", self.config.active_job.to_string())}
            _ => {}
        }
    }

    pub fn new(args: Vec<String>, intent: Subcommands, config: Config) -> Command {
        Command {
            args: args,
            intent: intent,
            config: config,
        }
    }
}

/*
pub enum Intent {
    ChangeActive(u32),
    MakeNote(String),
    MakeNoteOnJob(String, u32),
    PrintNotes(u32),
    GetCurrentJob,
    NoCmd,
}
*/

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    /// Makes a note under the active job
    Mknote {
        note: String,
        #[arg(short = 'j', long = "job")]
        job: Option<u32>
    },

    /// Changes the active job
    Chactive {
        jobnumber: u32
    },

    /// Prints the active order number
    Active,

    /// Prints the path where Workjournal looks for its config file
    Configpath,

    /// Prints the notes for a given job number
    Print {
        jobnumber: u32
    }
}

pub fn configpath() {
    let dirs = ProjectDirs::from("com", "SwissArmyWrench", "Workjournal").unwrap(); // SAFE
    let mut config_path = dirs.config_dir().to_owned();
    config_path.push("config.yaml");
    println!("Workjournal expects its configuration file to be located at:\n{}", config_path.display());
}

fn change_job_yaml(newjob: u32) {
    // Get the yaml config as a string
    let dirs = ProjectDirs::from("com", "SwissArmyWrench", "Workjournal").unwrap(); // SAFE
    let mut config_path = dirs.config_dir().to_owned();
    config_path.push("config.yaml");
    let config_string =
        std::fs::read_to_string(&config_path).expect("Unable to open config file to change jobs");

    // Regex to match on the key:value pair in the YAML
    let regex = Regex::new(r"active_job: (?<number>\d{1,5})").unwrap();
    let after = regex.replace_all(
        &config_string,
        format!("active_job: {}", newjob.to_string()),
    );
    // println!("{}", after);
    match std::fs::remove_file(&config_path) {
        Ok(_) => {}
        Err(_) => {
            println!("Unable to remove config file in order to re-write");
        }
    }
    let mut config_file = File::create(&config_path).expect("Unable to re-write new config");
    let _ = config_file.write(after.as_bytes());
}

fn grep_as_lines(path: PathBuf, query: String) -> Vec<String> {
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
    let sink = grep::searcher::sinks::UTF8(|_line_number, line| {
        match matcher.find(line.as_bytes()).unwrap() {
            Some(_) => {
                matches.push(trim_newline(line.to_string()));
            }
            None => {}
        }
        Ok(true)
    });

    let _out = searcher.search_path(&matcher, &path, sink);
    // println!("{:?}", out);
    // return matches after running the search
    matches
}

fn trim_newline(mut string: String) -> String {
    if string.ends_with("\n") {
        string.pop();
        if string.ends_with("\r") {
            string.pop();
        }
    }
    string
}

fn get_paths(dir: &PathBuf) -> Vec<String> {
    let mut ls: Vec<_> = read_dir(dir)
        .expect("Unable to read file system")
        .map(|opt| opt.unwrap().path().to_string_lossy().into_owned())
        .collect();
    ls.natural_sort::<str>();
    ls
}
