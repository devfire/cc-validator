// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // todo!()

    // first, let's remove all whitespace
    let cc: String = cc_number.split_whitespace().collect();
    println!("With whitespace removed: {}", cc);

    if !cc.chars().all(char::is_numeric) {
        false // non numeric chars found, bailing
    } else if cc.len() < 2 {
        false // numbers under 2 digits, bailing
    } else if !last_digit(cc) == 0 {
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
    let reversed = cc.chars().rev();
    // println!("Reversed: {}", reversed);
    for (i, ch) in reversed.into_iter().enumerate() {
        let mut n: i32 = ch as i32 - 0x30; // "cast" a char as an int

        if i % 2 != 0 {
            
            // we only process odd digits
            n = n * 2; // double every odd digit
            print!("position: {} char: {} ", i, n);
            
            // if the doubled number is two digits, we need to sum the digits
            // https://www.geeksforgeeks.org/program-for-sum-of-the-digits-of-a-given-number/
            if n > 9 {
                // println!("Two digits detected, adding");
                let added = sum_of_digits(n);
                println!("summed: {}", added);
            } // single digit, proceed with n
            
        }
    }

    1
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

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

#[allow(dead_code)]
fn main() {
    // let cc: &str = "4539 3195 0343 6476";
    // // let reversed = remove_whitespace(&cc);

    // let reversed_str = remove_whitespace(&cc);

    // while let Some(ch) = reversed_str.chars().next() {
    //     // reversed_str.nth(1);
    //     print!("{}", ch);
    // }
    luhn("4539 3195 0343 6476");
}
