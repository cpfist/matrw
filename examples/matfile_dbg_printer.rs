use matrw::load_matfile;
use std::env;
use std::fs::metadata;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need exactly one argument");
    }

    println!("Loading MAT-file from {}", &args[1]);

    let metadata = metadata(&args[1]);
    if !metadata.expect("Not a valid path").is_file() {
        panic!("Not a valid file");
    }

    let matfile = load_matfile(&args[1]);
    println!("{:#?}", matfile);
}
