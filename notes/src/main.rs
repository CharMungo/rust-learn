use std::io;
use std::fs::*;

fn main() {
    pick_file()
}

fn pick_file() {
    let files: Vec<_> = read_dir("/home/mungo/.todo/").expect("Failed to get directory").collect();
    let mut i = 1;
    for file in &files {
        let entry = file.as_ref().expect("Failed to create enteries");
        println!("{}: {}",i, entry.file_name().to_string_lossy());
        i += 1;
    }
    println!("Pick file");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if input.trim() == "new" || input.trim() == "n" {
        new_file()
    } else {
        let num: usize = input.trim().parse().expect("Failed to parse input");
        let note = &files[num - 1].as_ref().expect("huh?").path();
        let cont = read_to_string(note).expect("Failed to open file"); 
        println!("{}", cont.trim());
    }

}

fn new_file() {
    println!("Making new file");
}
