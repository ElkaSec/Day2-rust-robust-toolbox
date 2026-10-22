fn mean(a: &[i32]) -> Option<f64> {
    let len = a.len();
    if len == 0 {
        None
    } else {
        let mut mean = 0.0;
        for nb in a {
            mean += *nb as f64;
        }
        let len = len as f64;
        Some(mean / len)
    }
}

fn median(values: &[i32]) -> Option<f64> {
    let len = values.len();
    if len == 0 {
        None
    } else {
        let mut new_values: Vec<&i32> = values.iter().collect();
        new_values.sort();
        if len % 2 != 0 {
            return Some(*new_values[len / 2] as f64);
        } else {
            let mid = len / 2;
            Some((new_values[mid] + new_values[mid + 1]) as f64 / 2.00)
        }
    }
}

fn main() {
    let _data: [i32; 5] = [4, 1, 3, 7, 9];

    let mean = match mean(&_data) {
        Some(n) => n,
        None => {
            eprintln!("erreur data");
            std::process::exit(1);
        }
    };

    let median = match median(&_data) {
        Some(n) => n,
        None => {
            eprintln!("erreur data");
            std::process::exit(1);
        }
    };

    println!("data :{:?}", _data);
    println!("mean : {}", mean);
    println!("median : {}", median);
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn mean_of_three() {
        assert_eq!(mean(&[3, 2, 1]), Some(2.0), "must be 2");
    }
    #[test]
    fn mean_of_empty_is_none() {
        assert_eq!(mean(&[]), None, "must be None");
    }
    #[test]
    fn median_odd_list() {
        assert_eq!(median(&[4, 1, 3, 7, 9]), Some(4.0), "must equal 4");
    }
    #[test]
    fn median_even_list() {
        assert_eq!(median(&[4, 1, 3, 7, 9, 9]), Some(8.0), "must be equl 8");
    }
    #[test]
    fn median_of_empty_is_none() {
        assert_eq!(median(&[]), None, "must be equl 8");
    }
}
