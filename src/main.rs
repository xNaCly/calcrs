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
    let args = env::args().collect::<Vec<_>>();
    let input = args.get(1).expect("Not enough arguments, wanted filename");
    let filepath = env::current_dir()
        .expect("Failed to get cwd")
        .as_path()
        .join(input);

    let file = File::open(filepath).expect("Error opening file");
    let mut l = lexer::Lexer::new(BufReader::new(file).lines());

    // TODO: adjust once parser is done
    let _ = l.lex();
    // let mut m = parser::Parser::new(token);
    // let ast = m.parse();

    let ast = expr::Number {
        t: token::TokenType::NUMBER(12.5),
    };

    let mut allocator = alloc::Allocator::new();
    let mut pool = alloc::Pool::new();
    let mut code = ast.compile(&mut allocator, &mut pool);

    code.push(vm::Operation::Debug);
    code.push(vm::Operation::Argument(0));

    let mut vm = vm::Vm::new(&pool, code);
    vm.run();
}
