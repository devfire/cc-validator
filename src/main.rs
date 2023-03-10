// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // first, let's remove all whitespace
    let cc: String = cc_number.split_whitespace().collect();
    println!("With whitespace removed: {}", cc);

    /*
    Failure conditions:
    - non numeric characters
    - length less than 2
    - last digit is not zero
    */
    if !cc.chars().all(char::is_numeric) || cc.len() < 2 || last_digit(cc) > 0 {
        false
    } else {
        true
    }
}

fn sum_of_digits(mut number: i32) -> i32 {
    let mut sum = 0;
    while number != 0 {
        sum += number % 10;
        number = number / 10;
    }
    sum // return sum of digits
}

fn last_digit(cc: String) -> i32 {
    //first, let's reverse the string.
    let reversed = cc.chars().rev();
    let mut sum_of_all_digits = 0; //init the sum to 0

    for (i, ch) in reversed.into_iter().enumerate() {
        let mut n: i32 = ch as i32 - 0x30; // "cast" a char as an int

        if i % 2 != 0 {
            // check for odd numbered position
            n = n * 2; // double every odd digit
        }

        // if the doubled number is two digits, we need to sum the digits before adding
        sum_of_all_digits += sum_of_digits(n);
    }
    // return the last digit
    sum_of_all_digits % 10
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

// #[allow(dead_code)]
fn main() {
    luhn("8273 1232 7352 0569");
}
