fn check_input(value: &String) -> bool {
    if value.is_empty() == true {
        return false;
    }

    for char in value.chars() {
        let result = char.to_string().parse::<u32>();
        if result.is_err() {
            return false;
        }
    }
    true
}

fn multiply(value: &str) -> u128 {
    value
        .chars()
        .fold(1, |acc, x| acc * x.to_string().parse::<u128>().unwrap())
}

fn find_number(value: &String, limit: usize) -> (&str, u128) {
    let mut max_slice: &str = "";
    let mut max_val = 0;
    let mut counter = limit;

    while counter < value.len() {
        let lower_bound = counter - limit;
        let upper_bound = counter;

        let curr_slice = &value[lower_bound..upper_bound];
        let val = multiply(curr_slice);
        if curr_slice.chars().count() == 10 && val > max_val {
            max_val = val;
            max_slice = curr_slice;
        }
        counter += 1;
    }
    (max_slice, max_val)
}

pub fn init(value: String, limit: usize) -> () {
    let result = check_input(&value);

    match result {
        false => {
            panic!("Value is not a number")
        }
        _ => (),
    }
    let (max_str, max_val) = find_number(&value, limit);

    println!("Max Multiplication Value: {max_val}; String Value: {max_str}")
}

#[cfg(test)]
mod tests {
    use crate::api::api::{check_input, find_number, multiply};

    #[test]
    fn test_check_input_true() -> () {
        let val = "13421242".to_string();
        let result = check_input(&val);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_input_false() -> () {
        let val = "something".to_string();
        let result = check_input(&val);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_input_empty() -> () {
        let val = "".to_string();
        let result = check_input(&val);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_mult() -> () {
        let val = "123456".to_string();
        let result = multiply(&val);
        assert_eq!(result, 720);
    }

    #[test]
    fn test_check_mult_complex() -> () {
        let val = "7385342344".to_string();
        let result = multiply(&val);
        assert_eq!(result, 967680);
    }

    #[test]
    fn test_check_mult_zero() -> () {
        let val = "3423420414".to_string();
        let result = multiply(&val);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_default_string() -> () {
        let val =
            "798853072792707819458384342342349704396309603640976205270626701969153".to_string();
        let result = find_number(&val, 10);

        assert_eq!(result.0, "7819458384");
        assert_eq!(result.1, 7741440);
    }
}
