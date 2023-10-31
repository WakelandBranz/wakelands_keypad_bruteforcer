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


        let numbers = utils::v_from_char_to_int(&input);
        println!("Input -> {:?}", &numbers);
        utils::sleep(500);
            

        let combination_count = utils::get_permutation_count(&input);
        let combinations = bf.get_combos(numbers);
        

        println!("Possible combination count -> {}", combination_count);
        println!("Combinations -> {:?}", &combinations);
    }

}
