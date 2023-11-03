mod utils;

// bring all functions into scope
use crate::utils::filestream::*;

// TODO: heap's algorithm doesn't seem to work with n < 4 with input length < 3 so
// figuring out a solution for that would be nice.  I might ask stackoverflow


fn main() {
    // DEBUG VARIABLE
    let debug: bool = false;
    let completed: bool = false; // function to be implemented in the future
    let path: &'static str = "heap_data.txt";

    println!("[!] Press ctrl + c to exit at any time");

    while !completed {

        // reset file and check
        match reset_file(path) {
            true => if debug {println!("Successfully reset file")}
            _ => {
                if debug {println!("Failed to reset file")}
                return;
            }
        }


        println!("Please input the digits you have found on the keypad.");

        let input: Vec<char> = match utils::get_user_input() {
            Some(res) => res,
            None => {
                println!("[-] Invalid code input, please try again.");
                continue;
            }
        };


        let mut numbers = utils::convert_char_to_usize(&input);
        if debug {println!("Input -> {:?}", &numbers)}
        utils::sleep(500);
        
        utils::determine_operations(&mut numbers);

        let combinations = utils::filestream::parse_data_from_file(path);
        
        if debug {println!("Combinations -> {:?}", &combinations);}
        utils::print_parsed_data(&combinations);
    }

}
