use clap::{Arg, Command};
use std::fs;
use snlc::utils::gen_ir_file;
use snlc_ast::token::{Token, TokenKind};
fn main() {
    //use clap mod to read user input file path
    let matches = Command::new("snlc")
        .version("0.1")
        .author("yurzi")
        .about("compiler for snl")
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

    //get file basename
    let src_file_name = input_file_path.split('/').last().unwrap_or_else(|| {
        eprintln!("Error: invalid input file path");
        std::process::exit(1);
    });

    // remove extension name
    let src_file_name = src_file_name.split('.').next().unwrap_or_else(|| {
        eprintln!("Error: invalid input file path");
        std::process::exit(1);
    });

    let target_file_name = format!("src/bin/{}.rs", src_file_name);


    let input_file = fs::read_to_string(input_file_path).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(2);
    });

    let tokens = Token::from_str(input_file.as_str());
    // if a token is keyword, then add 'r#' before to its lexeme
    let tokens = tokens.into_iter().map(|token| {
        let mut lexeme = token.lexeme;
        if token.kind == TokenKind::Keyword {
            lexeme.insert_str(0, "r#");
        }
        Token::new(token.kind, token.pos, lexeme)
    }).collect::<Vec<Token>>();

    let pre_process_file = Token::to_str(tokens);
    let ir_file = gen_ir_file(pre_process_file);

    fs::write(target_file_name, ir_file).unwrap();
}
