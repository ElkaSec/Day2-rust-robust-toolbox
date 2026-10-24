pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}
pub fn mul(left: i32, right: i32) -> i32 {
    left * right
}
pub fn div(left: i32, right: i32) -> i32 {
    if right == 0 {
        return 0;
    } else {
        left / right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_sub() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_mul() {
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_div() {
        let result = div(8, 2);
        assert_eq!(result, 4);
    }
}
