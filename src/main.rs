// Rcat
// Liz Hardee October 16 2024

// Simple copy of the cat command written in rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    // grab arguments from command line
    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;
    // steps through provided filenames, reading the contents
    while i < args.len() {
        read_file(args[i].to_string());
        i+=1;
    }
}

// reads contents from files
fn read_file(filename: String) {
    // get filepath
    let path: &Path = Path::new(&filename);
    let display = path.display();

    // open file as read-only
    let mut file = match File::open(&path) {
        Err(why) =>  panic!("couldn't open {}, {}", display, why),
        Ok(file) => file,
    };

    // read file contents out to the command line
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{}:\n{}", display, contents),
    }
} 