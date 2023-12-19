
use std::io::Read;

use clap::{App, Arg, ArgMatches};

fn openfile(file_name: &str) -> Result<String, std::io::Error>{
    let mut content = String::new();
    let mut file = std::fs::File::open(file_name)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_option<'n, 'a>() -> ArgMatches<'n, 'a>{
    let args = App::new("rcat")
    .version("0.1")
    .about("another impl of cat")
    .arg(
        Arg::with_name("filename")
        .help("Open File name")
        .takes_value(false)
        .required(false)
    ).get_matches();
    args
}

#[cfg(test)]
mod openfile_test{
    use crate::openfile;

    #[test]
    fn openfile_test(){
        let content = openfile("test_source/test.txt").unwrap();
        assert_eq!(content, "this is a test line");
    }
}

fn main() {
    let args = get_option();
    let file_name = args.value_of("filename").unwrap();
    let content = openfile(file_name).unwrap();
    println!("{}", content);
}
