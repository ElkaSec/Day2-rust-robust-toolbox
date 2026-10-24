use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("./{} value1 <opt> value2", args[0]);
        return;
    }
    let value1 = args[1].parse::<i32>().expect("error parsing");
    let value2: i32 = args[3].parse::<i32>().expect("error parsing");
    let result = match args[2].as_str() {
        "+" => math::add(value1, value2),
        "-" => math::sub(value1, value2),
        "*" => math::mul(value1, value2),
        "/" => math::div(value1, value2),
        _ => {
            eprintln!("error parsing");
            return;
        }
    };
    println!("{}", result)
}
