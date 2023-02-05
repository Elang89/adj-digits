fn check_input(value: &String) -> bool {
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
