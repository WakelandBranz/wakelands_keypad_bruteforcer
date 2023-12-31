use rand::Rng;
use std::{thread, time};

pub mod filestream;

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
    if vec.len() == 4 && contains_repeating_digits(&convert_char_to_u64(&vec)) {
        println!("[!] Repeating digits within a 4 digit code, impossible scenario");
        return None;
    }

    return Some(vec);
}

/*
 * I apologize in advance for how unnecessary half of these conversion functions
 * are. I am trying my best to write this in an efficient fashion but honestly
 * it is a struggle to write this all out by hand.  Its definitely stupid
 * but it works.
 */

// iterates over each char and turns it into a u64 using map
pub fn convert_char_to_u64 (input: &Vec<char>) -> Vec<u64> {
    return input
    .iter()
    .map(|c| c.to_digit(10).unwrap()  as u64) // 
    .collect();
}

// iterates over each char and turns it into a u64 using map
// MUST CONSIST OF CHARS THAT CAN BE CONVERTED TO AN UNSIGNED INTEGER NATURALLY
pub fn convert_char_to_usize (input: &Vec<char>) -> Vec<usize> {
    return input
    .iter()
    .map(|c| c.to_digit(10).unwrap()  as usize)
    .collect();
}

// vector of u64 to String
pub fn convert_u64_to_string (input: &Vec<u64>) -> String {
    return input
    .iter()
    .map(|&x| (x as u8 + b'0') as char)
    .collect::<String>();
}

// iterates over each u64 and converts them to a usize using map
pub fn convert_u64_to_usize (input: &Vec<u64>) -> Vec<usize> {
    return input
    .iter()
    .map(|n| *n as usize)
    .collect();
}

pub fn convert_usize_to_string (input: &Vec<usize>) -> String {
    return input
        .into_iter()
        .map(|u| u.to_string())
        .collect();
}

pub fn sleep (ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
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
    let mut clone = input.clone(); // TODO: figure out why this function does not work without the clone, rust noobie stuff

    // dedup only removes repeating sequential digits, so 
    // sorting them prior to using dedup yields the desired result
    clone.sort();
    clone.dedup();

    if clone.len() == original_length {
        return false;
    }
    return true;
}

// heap's algorithm implementation with time complexity of O(n) (fast)
// directly modifies the value you pass into it, so make a copy prior.
// partially referenced https://gist.github.com/RichardJohnn/8e6af62e7272cf39e8b6 and https://en.wikipedia.org/wiki/Heap%27s_algorithm
// k is how many digits you want in your resulting vector, a is our array we pass in, new receieves all data

// currently not functional with size < 4, not sure why...
pub fn generate(n : usize, a : &mut Vec<usize>) {
    if n == 1 {
        //println!("{:?}", a);
        filestream::append_file("heap_data.txt", convert_usize_to_string(a).as_str());
    }
    else {
        for i in  0 .. n - 1 {
            generate(n - 1, a);

            if n % 2 == 0 {
                a.swap(i, n - 1);
            }
            else {
                a.swap(0, n - 1);
            }
        }
        generate(n - 1, a);
    }
}

pub fn print_parsed_data (input: &Vec<String>) {
    for i in 0 .. input.len() {
        println!("{} -> {}", i + 1, input[i])
    }
}