
use std::io::Read;

use clap::{App, Arg};

fn main() {
    let args = App::new("rcat")
    .version("0.1")
    .about("another impl of cat")
    .arg(
        Arg::with_name("filename")
        .help("Open File name")
        .takes_value(false)
        .required(false)
    ).get_matches();
    let file_name = args.value_of("filename").unwrap();
    let mut file = std::fs::File::open(file_name).unwrap();
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}
