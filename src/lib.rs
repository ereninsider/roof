pub fn sum(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_sum_negative() {
        let result = sum(2, -4);
        assert_eq!(result, -2);
    }
}