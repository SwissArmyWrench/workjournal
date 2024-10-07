use std::env;
use workjournal::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    workjournal::wip();
    
    let mut intent: workjournal::Intent = workjournal::Intent::NoCmd;
    if args[1] == "chactive" {
        intent = workjournal::Intent::ChangeActive(args[2].parse::<u32>().unwrap());
        println!("chactive success!");
    } else if args[1] == "mknote" {
        println!("mknote success!");
        let note = env::args().skip(2).collect::<Vec<String>>().join(" ");
        println!("{}", &note);
        intent = workjournal::Intent::MakeNote(note);
    }
    
}
