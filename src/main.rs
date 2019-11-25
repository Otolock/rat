extern crate clap;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("rat")
        .version("1.0")
        .author("Antonio Santos <antonio@antoniosantos.io>")
        .about("Writes the contents of the input file to the standard output.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{}", contents);
}
