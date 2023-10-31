pub mod utils;

use rand::Rng;

pub struct Bruteforce {
    debug: bool,
}

impl Bruteforce {
    pub fn new (is_debugging: bool) -> Self {
        Self {
            debug: is_debugging,
        }
    } 

    // gets user input, returns trimmed user input with length, none is a simple error
    pub fn get_user_input (&self) -> Option<Vec<char>> {
        let mut user_input: String = String::new(); // stores input once acquired in input_size
        std::io::stdin() // read input
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // remove whitespaces and other unnecessary stuff, please let me know if there is a better way to do this
        let user_input: String = user_input.replace(" ", "",).replace("\n", "").replace("\r", "");

        // if user input is empty
        if user_input == "" {
            if self.debug {println!("[!] Empty String")}
            return None;
        }
        // if greater than maximum allowed amount of integers
        if user_input.chars().count() > 4 { 
            if self.debug {println!("[!] String size exceeds limit, length -> {}", user_input.chars().count())}
            return None;
        }

        let vec: Vec<char> = user_input.chars().collect();

        return Some(vec);
    }

    // thanks https://stackoverflow.com/questions/59206653/how-to-calculate-21-factorial-in-rust
    fn factorial (&self, input: u64) -> u64 {
        return (1..=input).product();
    }

    // thanks https://stackoverflow.com/questions/65561566/number-of-combinations-permutations
    pub fn get_permutation_count (&self, vec: &Vec<char>) -> u64 {
        let n: u64 = vec.len() as u64;
        return (n - n + 1..=n).product();
    }

    fn get_combos (&self, combos: Vec<u64>) -> u64 {
        match combos.len() {
            4 => {
                return self.combos_len_4(combos);
            }
            3 => {
                todo!();
            }
            2 => {
                todo!();
            }
            1 => {
                return combos[0]
            }
            _ => panic!("Check get_combos...")
        }
    }

    fn combos_len_4 (&self, combos: Vec<u64>) -> u64 {

        // recursive function
        todo!();

        return 1231
    }

}