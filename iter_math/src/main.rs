use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let list: Vec<i32> = match args.iter().map(|x| x.parse::<i32>()).collect() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error");
            std::process::exit(2);
        }
    };
    let max = list.sum();
}
