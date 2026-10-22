use std::env;
use std::process::exit;

struct Report<'a> {
    winner: &'a str,
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("error: expected 2 arguments: <a> <b>");
        exit(1);
    }
    let a = &args[1];
    let b = &args[2];

    let r = Report {
        winner: longest(a, b),
    };

    println!("longest {} ({}) chars", r.winner, r.winner.chars().count())
}
