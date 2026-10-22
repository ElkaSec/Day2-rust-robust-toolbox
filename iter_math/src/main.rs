use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let sum: i32 = args
        .iter()
        .map(|x| x.parse::<i32>().expect("error: invalid number '<arg>' "))
        .sum();

    let evens = args
        .iter()
        .map(|x| x.parse::<i32>().expect("error: invalid number '<arg>' "))
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>();

    let squares = args
        .iter()
        .map(|x| x.parse::<i32>().expect("error: invalid number '<arg>' "))
        .map(|x| x * x)
        .collect::<Vec<i32>>();

    println!("sum : {}", sum);
    println!("evens : {:?}", evens);
    println!("{:?}", squares);
}
