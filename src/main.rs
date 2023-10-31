mod bruteforce;

use crate::bruteforce::*; // Bring Bruteforce into scope

fn main() {
    // DEBUG VARIABLE
    let debug: bool = true;
    let completed: bool = false;

    let bf: Bruteforce = Bruteforce::new(debug);

    println!("[!] Press ctrl + c to exit at any time");

    while !completed {
        println!("Please input the digits you have found on the keypad.");

        let input: Vec<char> = match bf.get_user_input() {
            Some(res) => res,
            None => {
                println!("[-] Invalid code input, please try again.");
                continue;
            }
        };


        let input_s: String = input.iter().collect();
        println!("Input -> {}", &input_s);

        let numbers = utils::v_from_char_to_int(&input);

        let combination_count = bf.get_permutation_count(&input);
        let combinations = todo!();
        

        println!("Possible combination count -> {}", combination_count);
        println!("Combinations -> {:?}", &combinations);
    }

}
