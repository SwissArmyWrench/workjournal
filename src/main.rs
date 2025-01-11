use clap::Parser;
use std::env::args;
use workjournal::Subcommands;

#[derive(Parser, Debug)]
#[command(version = "1.1.0", about = "Keeping notes on your workday, easily, in the terminal")]
struct CmdIn {
    #[command(subcommand)]
    command: Subcommands,
    
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
