mod lexer;
mod parser;
mod util;
mod eval;
mod simple_types;

use std::io;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = String::new();
    loop {
        print!("> ");
        stdout.flush();
        buf.clear();
        stdin.read_line(&mut buf);
        let toks = match lexer::lex(buf.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        };
        let instrs = match parser::parse_expr(toks) {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        };
        let out = eval::eval_expr(instrs);
        println!("{:?}", out);
    }
}