use rand::Rng;
use std::{thread, time};

// vector from chars to ints
// takes each char in a vector of chars, iterates
// through them, maps each char and sets char to what it
// actually represents as a u64
pub fn v_from_char_to_int (input: &Vec<char>) -> Vec<u64> {
    return input
    .iter()
    .map(|c| c.to_digit(10).unwrap() as u64)
    .collect();
}

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