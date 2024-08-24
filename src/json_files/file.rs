use std::fs::File;
use std::io::Read;

pub fn get_json_text(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?; // Unwrap the Result using ?
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read the file contents into the string
    Ok(contents)
}
