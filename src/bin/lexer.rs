use clap::{Arg, Command};
use std::fs;

use snlc_lexer::tokensize;

fn main() {
    let matches = Command::new("snlc lexer")
        .version("0.1")
        .author("yurzi")
        .about("lexer for snlc")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file_path = matches.get_one::<String>("INPUT").unwrap_or_else(|| {
        eprintln!("Error: no input file specified");
        std::process::exit(1);
    });

    let input_file = fs::read_to_string(input_file_path).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(2);
    });

    let res: String = tokensize(&input_file)
        .map(|token| format!("{:?}\n", token))
        .collect();
    println!("{res}");
}
