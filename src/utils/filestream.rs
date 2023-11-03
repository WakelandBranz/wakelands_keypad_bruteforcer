use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// https://stackoverflow.com/questions/65375081/how-do-i-write-multiple-strings-into-a-file-without-them-overwriting-each-other
// https://www.includehelp.com/rust/append-data-into-an-existing-file.aspx
pub fn append_file (path: &'static str, input_data: &str) -> bool {
    let mut f = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .expect("Unable to open file");

    f.write_all(input_data.as_bytes()).expect("Unable to write data to file");

    return true;
}

pub fn create_file (path: &'static str) -> Result<fs::File, &'static str> {
    if !path_exists(path) {
        return Ok(fs::File::create(path).unwrap());
    }
    return Err("File already exists");
}

pub fn path_exists (name: &str) -> bool {
    fs::metadata(name).is_ok()
}

// iunno how Result<(), Box<dyn Error>> works so I'm just going to return a String
pub fn read_file (path: &'static str) -> String {
    return fs::read_to_string(path).unwrap();
}

// delete file then create the same file
pub fn reset_file (path: &'static str) -> bool {
    fs::remove_file(path).expect("File failed to be removed in clear_file");

    match create_file(path) {
        Ok(_) => return true,
        _ => return false
    }
}

// iterates through all data in heap_data.txt, pushes each 4 digits to a vector,
// pushes that concatenated vector into a resultant vector, sorts it,
// removes duplicates, and returns.
pub fn parse_data_from_file (path: &'static str) -> Vec<String> {
    let data: String = read_file(path);
    let mut next: Vec<char> = Vec::new();
    let mut result: Vec<String> = Vec::new();
    for i in 0..data.len() {
        // push next &str as a u64
        next.push(data.as_str()[i..i+1].parse().unwrap());
        
        // if next has 4 digits, push concatenated next
        if next.len() == 4 {
            result.push(String::from_iter(&next));
            next.clear();
        }
    }
    result.sort();
    return result;
}