use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("one argument is needed !");
        return;
    }
    let mut levels: HashMap<&str, usize> = HashMap::new();

    let contents = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("error reading the file");
            return;
        }
    };

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            *levels.entry("other").or_insert(0) += 1;
            continue;
        }
        let level = parts[2];
        match level {
            "INFO" => *levels.entry("INFO").or_insert(0) += 1,
            "WARN" => *levels.entry("WARN").or_insert(0) += 1,
            "ERROR" => *levels.entry("ERROR").or_insert(0) += 1,
            _ => *levels.entry("other").or_insert(0) += 1,
        }
    }
    let count = contents.lines().count();

    let mut v: Vec<&&str> = levels.keys().collect();
    v.sort();
    println!("report for {}, ({} lines )", args[1], count);
    for elm in v {
        println!("{} :{}", elm, levels.get(elm).expect("never"));
    }
}
