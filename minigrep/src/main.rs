use std::env;
use std::fs;
use std::process::exit;

struct Config {
    query: String,
    path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let config = Config::build(&args);
    let config = match config {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Config error ");
            exit(2);
        }
    };
    let contents = match fs::read_to_string(config.path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Config error ");
            exit(2);
        }
    };
    if !ignore_case {
        let out = search(&config.query, &contents);
        println!("{}", out.join("\n"));
        exit(0);
    }
    let out = search_ignore_case(&config.query, &contents);
    println!("{}", out.join("\n"));
    exit(0);
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            out.push(line);
        }
    }
    out
}
pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            out.push(line);
        }
    }
    out
}
