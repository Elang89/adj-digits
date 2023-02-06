fn check_input(value: &str) -> bool {
    if value.is_empty() {
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

fn find_number(value: &str, limit: usize) -> (&str, u128) {
    let mut max_slice: &str = "";
    let mut max_val = 0;
    let mut counter = limit;

    while counter < value.len() {
        let lower_bound = counter - limit;
        let upper_bound = counter;

        let curr_slice = &value[lower_bound..upper_bound];
        let val = multiply(curr_slice);
        if curr_slice.chars().count() == limit && val > max_val {
            max_val = val;
            max_slice = curr_slice;
        }
        counter += 1;
    }
    (max_slice, max_val)
}

pub fn init(value: String, limit: usize) {
    let result = check_input(&value);

    if let false = result {
        panic!("Value is not a number")
    }

    let (max_str, max_val) = find_number(&value, limit);

    println!("Max Multiplication Value: {max_val}; String Value: {max_str}")
}

#[cfg(test)]
mod tests {
    use crate::api::numbers::{check_input, find_number, multiply};

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

    #[test]
    fn test_limit_four() -> () {
        let val = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string();
        let result = find_number(&val, 4);

        assert_eq!(result.0, "9989");
        assert_eq!(result.1, 5832);
    }
}
