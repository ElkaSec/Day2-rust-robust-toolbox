fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut max = list.first()?;
    for l in list {
        if l > max {
            max = l;
        }
    }
    Some(max)
}
fn main() {
    let list = [1, 2, 3];
    let floats = [1.0, 2.0, 3.0];
    let fruits: [&str; 3] = ["pear", "apple", "fig"];
    let nbr = match largest(&list) {
        Some(v) => &v.to_string(),
        None => "no item found ",
    };
    let pear = match largest(&fruits) {
        Some(v) => &v.to_string(),
        None => "no item found ",
    };
    let flt = match largest(&floats) {
        Some(v) => &v.to_string(),
        None => "no item found ",
    };
    println!("{} , {}, {}", nbr, pear, flt);
}
