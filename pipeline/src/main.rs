use std::env;
use std::process;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let path = args.remove(0);
    if args.len() < 2 {
        eprintln!("./{}  -int-  - inc/double/square/flip  ", path);
        return;
    }
    let mut number = match args.remove(0).parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("parse error first args");
            return;
        }
    };

    for operation in args {
        let func = match build(&operation.as_str()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        number = func(number);
    }
    println!("result : {}", number);
}

fn build(op: &str) -> Result<fn(i32) -> i32, String> {
    match op {
        "inc" => Ok(inc),
        "double" => Ok(double),
        "square" => Ok(square),
        "flip" => Ok(flip),
        _ => Err(format!("parsing operation error ")),
    }
}

fn inc(n: i32) -> i32 {
    n + 1
}
fn double(n: i32) -> i32 {
    2 * n
}
fn square(n: i32) -> i32 {
    n * n
}
fn flip(n: i32) -> i32 {
    -n
}
