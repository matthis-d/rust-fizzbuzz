pub fn fizzbuzz(input: i64) -> String {
    match input {
        y if y % 15 == 0 => String::from("FizzBuzz"),
        y if y % 5 == 0 => String::from("Buzz"),
        y if y % 3 == 0 => String::from("Fizz"),
        _ => format!("{}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
    }

    #[test]
    fn fizz_for_3() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn buzz_for_5() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
    }

    #[test]
    fn fizzbuzz_for_3_and_5() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }
}
