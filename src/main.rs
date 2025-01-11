use clap::{Parser, Subcommand};
use std::env::args;

#[derive(Parser, Debug)]
#[command(version = "1.1.0", about = "Keeping notes on your workday, easily, in the terminal")]
struct CmdIn {
    #[command(subcommand)]
    command: Subcommands,
    
}

#[derive(Debug, Clone, Subcommand)]
enum Subcommands {
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


fn main() {
    let intent = match CmdIn::parse().command {
        Subcommands::Mknote {note, job} => {workjournal::Intent::MakeNoteOnJob(note, job.unwrap())},
        Subcommands::Chactive {jobnumber} => {workjournal::Intent::ChangeActive(jobnumber)},
        Subcommands::Active => {workjournal::Intent::GetCurrentJob},
        Subcommands::Configpath => {
            workjournal::configpath();
            workjournal::Intent::NoCmd},
        Subcommands::Print {jobnumber} => {workjournal::Intent::PrintNotes(jobnumber)},

        // _ => todo!()
    };
    workjournal::Command::new( 
        args().collect(), 
        intent,
        workjournal::Config::load().expect("Error loading config")
        ).run()

}
