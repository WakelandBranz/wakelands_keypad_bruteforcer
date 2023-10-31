pub mod utils;

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

    pub fn get_combos (&self, combos: Vec<u64>) -> Vec<u64> {
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
                todo!();
            }
            _ => panic!("Check get_combos...")
        }
    }

    fn combos_len_4 (&self, combos: Vec<u64>) -> Vec<u64> {
        // next stores newly generated unique vectors of u64 for easy comparisons
        // buffer stores new digit to either discard or push into next
        // result stores unique values
        let mut next: Vec<u64> = Vec::new();
        let mut buffer: u64;
        let mut result: Vec<u64> = Vec::new();

        // while we haven't generated all 24 unique digits
        while result.len() != 24 {

            // while we haven't matched our desired length, generate a new integer 
            // and push it to the back of our new vector we want to add
            while next.len() != 4 {
                if self.debug {println!("Next Length {}", &next.len());}

                // generate new buffer, check if it is already within the new vector
                buffer = utils::gen_next(&combos);
                if self.debug {println!("Buffer -> {}", &buffer)}
                if next.contains(&buffer) {
                    continue;
                }
                // if it isn't, add it to our stack
                else {
                    next.push(buffer);
                }
            }

            if self.debug {println!("[!] Next length is now 4, comparing.")}

            // stupid variable name contains next 4 digit integer to compare
            let geeking: u64 = utils::concat(&next);

            if result.contains(&geeking) {
                next.clear();

                if self.debug {println!("Non-unique result *{}* in *{:?}*", geeking, &result)}
                utils::sleep(1);
                continue;
            }
            else {
                result.push(geeking);
            }
        }

        result.sort();
        return result;
    }
}