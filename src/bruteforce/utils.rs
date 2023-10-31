// vector from chars to ints

// takes each char in a vector of chars, iterates
// through them, maps each char and sets the pointer
// to each char to a u64

pub fn v_from_char_to_int (input: &Vec<char>) -> Vec<u64> {
    return input
    .iter()
    .map(|c| *c as u64)
    .collect();
}