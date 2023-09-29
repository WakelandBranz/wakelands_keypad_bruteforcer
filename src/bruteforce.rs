pub struct Bruteforce {
    debug: bool
}

impl Bruteforce {
    pub fn new (is_debugging: bool) -> Self {
        Self {
            debug: is_debugging
        }
    } 

    // gets user input, returns trimmed user input with length, none is a simple error
    pub fn get_user_input (&self) -> Option<Vec<char>> {
        let mut user_input: String = String::new(); // stores input once acquired in input_size
        let input_size: usize = std::io::stdin() // read input
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

        let mut vec: Vec<char> = user_input.chars().collect();

        return Some(vec);
    }

    pub fn get_combinations (&self, vec: &Vec<char>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<char> = Vec::with_capacity(4);
        
        // iterate through first number
        for i in 0..vec.len() {
            // push first number to end
            stack.push(vec[i]);

            if stack.len() == 4 {
                // turns all chars into a string and collects them into one string.
                let combo = stack.iter().map(|n| n.to_string()).collect::<String>();

                // ensure value doesn't already exist
                if !result.contains(&combo) {
                    result.push(combo);
                }
            }
            else {
                for j in (i + 1)..vec.len() {
                    stack.push(vec[j]);

                    if stack.len() == 4 {
                        // turns all chars into a string and collects them into one string.
                        let combo = stack.iter().map(|n| n.to_string()).collect::<String>();
        
                        // ensure value doesn't already exist
                        if !result.contains(&combo) {
                            result.push(combo);
                        }
                    }

                    stack.pop();
                }
            }
            stack.pop();

            println!("Iteration {} complete, result -> {:?}", &i, &result);
        } 

        return result;
    }
}