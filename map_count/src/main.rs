use std::{env, fs};
// Import HashMap
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./{}  filename ", args[0]);
        return;
    }
    let contents = match fs::read_to_string(&args[1]) {
        Ok(n) => n,
        Err(_) => {
            println!("error: cannot read file '{}'", args[1]);
            return;
        }
    };

    let contents = contents.to_lowercase();
    let contents = contents.split_whitespace();
    let mut map: HashMap<&str, usize> = HashMap::new();

    for content in contents {
        *map.entry(content).or_insert(0) += 1;
    }

    let mut entries: Vec<(&str, usize)> = map.into_iter().collect();
    entries.sort();
    for (key, value) in entries {
        println!("{}:{}", key, value);
    }
}
