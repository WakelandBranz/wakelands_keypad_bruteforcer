use rand::Rng;
use std::{thread, time};

// gets user input, returns trimmed user input with length, none is a simple error
pub fn get_user_input () -> Option<Vec<char>> {
    let mut user_input: String = String::new(); // stores input once acquired in input_size
    std::io::stdin() // read input
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // remove whitespaces and other unnecessary stuff, please let me know if there is a better way to do this
    user_input = user_input.replace(" ", "",).replace("\n", "").replace("\r", "");

    // if user input is empty
    if user_input == "" {
        println!("[!] Empty String");
        return None;
    }
    // if greater than maximum allowed amount of integers
    if user_input.chars().count() > 4 { 
        println!("[!] String size exceeds limit, length -> {}", user_input.chars().count());
        return None;
    }

    let vec: Vec<char> = user_input.chars().collect();

    // this is a check if the input is 4 characters wide and there are repeating digits
    // that would be an illegal scenario
    if vec.len() == 4 && contains_repeating_digits(&convert_chars_to_u64(&vec)) {
        println!("[!] Repeating digits within a 4 digit code, impossible scenario");
        return None;
    }

    return Some(vec);
}

// vector from chars to ints
// takes each char in a vector of chars, iterates
// through them, maps each char and sets char to what it
// actually represents as a u64
pub fn convert_chars_to_u64 (input: &Vec<char>) -> Vec<u64> {
    return input
    .iter()
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect();
}

// generates the next digit to be compared based on a vector of u64s
pub fn gen_next (available: &Vec<u64>) -> u64 {
    let mut rng = rand::thread_rng();
    return available[rng.gen_range(0..available.len())];
}

// thanks https://stackoverflow.com/questions/58922609/how-do-i-concatenate-a-vector-of-integers-into-a-single-integer
pub fn concat(vec: &Vec<u64>) -> u64 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}

pub fn sleep (ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

// thanks https://stackoverflow.com/questions/59206653/how-to-calculate-21-factorial-in-rust
pub fn factorial (input: u64) -> u64 {
    return (1..=input).product();
}

// thanks https://stackoverflow.com/questions/65561566/number-of-combinations-permutations
// https://www.youtube.com/watch?app=desktop&v=XJnIdRXUi7A
pub fn get_permutation_count (vec: &Vec<char>) -> u64 {
    let n: u64 = vec.len() as u64;
    return (n - n + 1..=n).product();
}

// stores original vector's length, removes all (possibly) duplicate
// digits, then checks if their lengths are the same after the process.
// if so, then we return false because there are no repeating digits.
// otherwise return true, because there are repeating digits.
// this should be used with 4 long integers.
pub fn contains_repeating_digits (input: &Vec<u64>) -> bool {
    let original_length = input.len();
    let mut clone = input.clone(); // TODO: figure out why this function does not work without the clone

    clone.sort();
    clone.dedup();

    if clone.len() == original_length {
        return false;
    }
    return true;
}