use std::env;
use workjournal::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
}
