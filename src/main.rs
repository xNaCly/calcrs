use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod alloc;
mod expr;
mod lexer;
mod parser;
mod token;
mod types;
mod vm;
use expr::Node;

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Not enough arguments, wanted filename");
    let filepath = env::current_dir()
        .expect("Failed to get cwd")
        .as_path()
        .join(input);

    let file = File::open(filepath).expect("Error opening file");
    let mut l = lexer::Lexer::new(BufReader::new(file).lines());

    // TODO: adjust once parser is done
    let tokens = dbg!(l.lex());
    let mut m = parser::Parser::new(tokens);

    let mut allocator = alloc::Allocator::new();
    let mut pool = alloc::Pool::new();
    let mut codes = vec![];
    let ast = m.parse();
    dbg!(ast.len());
    for n in ast {
        let code = n.unwrap().compile(&mut allocator, &mut pool);
        codes.extend(code)
    }

    codes.push(vm::Operation::Debug);
    codes.push(vm::Operation::Argument(0));

    let mut vm = vm::Vm::new(&pool, dbg!(codes));
    vm.run();
}
