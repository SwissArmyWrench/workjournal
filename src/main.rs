use clap::Parser;
use std::env::args;
use workjournal::{Command, Subcommands, Config};

#[derive(Parser, Debug)]
#[command(version = "1.1.0", about = "Keeping notes on your workday, easily, in the terminal")]
struct CmdIn {
    #[command(subcommand)]
    command: Subcommands,
    
}




fn main() {
    Command::new( 
        args().collect(), 
        CmdIn::parse().command,
        Config::load().expect("Error loading config")
        ).run()
}
