use std::env;

fn average(mut nums_raw: Vec<String>) -> Result<f64, String> {
    nums_raw.remove(0);
    if nums_raw.len() == 0 {
        return Err("no args given".to_string());
    }
    let mut somme: f64 = 0.0;
    let len = nums_raw.len();
    for num in nums_raw {
        let s = match num.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(format!("invalid number: '{}'", num));
            }
        };
        somme += s;
    }
    return Ok(somme / len as f64);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match average(args) {
        Ok(n) => println!("the average is : {}", n),
        Err(n) => println!(" error :{}", n),
    }
}
