use std::env;
use std::fs;

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: more [filename]");
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", contents);
}
