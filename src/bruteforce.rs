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

    pub fn get_combos (&self, combos: Vec<u64>) -> Option<Vec<u64>> {
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

    fn combos_len_4 (&self, combos: Vec<u64>) -> Option<Vec<u64>> {
        // check for repeating digits
        if utils::contains_repeating_digits(&combos) {
            println!("[!] Your input is 4 digits long and contains repeating digits.  Please reinput a valid combination of characters.");
            return None;
        }

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

                if self.debug {println!("Non-unique result *{}* in *{:?}*", geeking, &result);}
                continue;
            }
            else {
                result.push(geeking);
            }
        }

        result.sort_unstable();
        return Some(result);
    }
}