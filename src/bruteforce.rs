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

    // typically I would make combos an &mut Vec<usize> but I want to 
    // return a new value and not modify it, so there may be a microsecond
    // performance decrease lol
    pub fn get_combos (&self, combos: &Vec<usize>) -> Vec<u64> {
        let result = utils::get_permutations(combos.len(), &combos);
        return result;
    }

    fn combos_len_4 (&self, combos: Vec<u64>) -> Option<Vec<u64>> {
        // next stores newly generated unique vectors of u64 for easy comparisons
        // buffer stores new digit to either discard or push into next
        // result stores unique values
        let mut next: Vec<u64> = Vec::new();
        let mut buffer: u64;
        let mut result: Vec<u64> = Vec::new();

        let mut total_iterations: u32 = 0;

        // while we haven't generated all 24 unique digits
        while result.len() != 24 {

            total_iterations += 1;

            // while we haven't matched our desired length, generate a new integer 
            // and push it to the back of our new vector we want to add
            while next.len() != 4 {
                // generate new buffer, check if it is already within the new vector
                buffer = utils::gen_next(&combos);
                if next.contains(&buffer) {
                    continue;
                }
                // if it isn't, add it to our stack
                else {
                    next.push(buffer);
                }
            }

            // contains next 4 digit integer to compare
            let combination: u64 = utils::concat(&next);

            if result.contains(&combination) {
                next.clear();

                if self.debug {println!("[-] Non-unique result *{}* in *{:?}*", combination, &result);}
                continue;
            }
            else {
                if self.debug {println!("[+] Unique result *{}* in *{:?}*", combination, &result);}
                result.push(combination);
            }
        }

        if self.debug {println!("[!] Found result after {} iterations!", total_iterations)}

        result.sort_unstable();
        return Some(result);
    }
}