use rand::Rng;

#[allow(dead_code)]
fn generate_random_number(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut number = String::new();

    for _ in 0..length {
        let digit: u8 = rng.gen_range(0..10);
        number.push_str(&digit.to_string());
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_number_creates_correct_length() {
        let length = 10;
        let number = generate_random_number(length);
        assert_eq!(number.len(), length);
    }

    #[test]
    fn generate_random_number_creates_only_digits() {
        let length = 10;
        let number = generate_random_number(length);
        assert!(number.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn generate_random_number_creates_different_numbers() {
        let length = 10;
        let number1 = generate_random_number(length);
        let number2 = generate_random_number(length);
        assert_ne!(number1, number2);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn generate_random_number_panics_on_zero_length() {
        let length = 0;
        let _ = generate_random_number(length);
    }
}