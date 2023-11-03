mod bruteforce;

use crate::bruteforce::*; // Bring Bruteforce into scope


fn main() {
    // DEBUG VARIABLE
    let debug: bool = true;
    let completed: bool = false; // function to be implemented in the future

    // initialize data storage file for heap's algorithm
    match utils::filestream::create_file("heap_data") {
        Ok(_) => if debug {println!("Successfully created heap_data file")},
        _ => if debug {println!("Failed to create heap_data file")}
    }

    let bf: Bruteforce = Bruteforce::new(debug);

    println!("[!] Press ctrl + c to exit at any time");

    while !completed {
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
            

        let combination_count = utils::get_permutation_count(&input);
        let combinations = bf.get_combos(&numbers);
        

        println!("Possible combination count -> {}", combination_count);
        println!("Combinations -> {:?}", &combinations);

        // testing
        utils::generate(numbers.len(), &mut numbers);
    }

}
