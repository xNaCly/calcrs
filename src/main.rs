use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod lexer;
mod token;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let input = args.get(1).expect("Not enough arguments, wanted filename");
    let filepath = env::current_dir()
        .expect("Failed to get cwd")
        .as_path()
        .join(input);

    let file = File::open(filepath).expect("Error opening file");
    let mut l = lexer::Lexer::new(BufReader::new(file).lines());
    l.lex();
}
