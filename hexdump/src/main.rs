use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./{} binaryfile", args[0]);
        exit(1);
    }
    let path = args.remove(1);

    let content: Vec<u8> = match fs::read(path) {
        Ok(n) => n,
        Err(_) => {
            println!("problem reading the file");
            exit(2);
        }
    };

    let mut mm: usize = 0;
    for chunk in content.chunks(16) {
        print!("  {:08X}  :", mm);
        for octet in chunk {
            print!("{:02X} ", octet)
        }

        print!("|");

        for octet in chunk {
            print!(" {}", convert(*octet));
        }
        println!("|");
        mm += 16;
    }
}

fn convert(octet: u8) -> char {
    if (32..127).contains(&octet) {
        octet as char
    } else {
        '.'
    }
}
