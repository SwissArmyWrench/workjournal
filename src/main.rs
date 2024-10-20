use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // workjournal::wip();
    
    let mut intent: workjournal::Intent = workjournal::Intent::NoCmd;
    if args.len() == 1 {
        println!("Interactive mode not yet ready!")
    } else if args[1] == "chactive" {
        intent = workjournal::Intent::ChangeActive(args[2].parse::<u32>().unwrap());
    } else if args[1] == "mknote" {
        let note = env::args().skip(2).collect::<Vec<String>>().join(" ");
        intent = workjournal::Intent::MakeNote(note);
    } else if args[1] == "print" {
        intent = workjournal::Intent::PrintNotes(args[2].parse::<u32>().unwrap());

    }

    workjournal::Command::new(args, intent, workjournal::Config::load().unwrap()).run();
    
}
