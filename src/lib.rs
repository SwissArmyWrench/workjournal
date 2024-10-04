// Lib file for Workjournal


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
    change_active(u32),
    make_note(String),
    


}
