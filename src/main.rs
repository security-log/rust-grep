use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args = App::new("Rust Grep")
        .version("1.0")
        .about("Searchs for patterns")
        .arg(
            Arg::with_name("pattern")
                .takes_value(true)
                .required(true)
                .help("The pattern to search."),
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .required(false)
                .help("This is the path for the file to read"),
        )
        .get_matches();

    let path = args.value_of("input").unwrap_or("-");

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    if path == "-" {
        // If not a file path == "-"
        let stdin = io::stdin();
        let reader = stdin.lock();

        process_lines(reader, re);
    } else {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
