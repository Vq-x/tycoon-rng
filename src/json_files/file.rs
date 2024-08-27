use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn get_json_text(file_name: &str) -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // This gets the root directory of your project
    path.push(file_name);
    let mut file = File::open(path.to_str().expect("the path given was invalid"))?; // Unwrap the Result using ?
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read the file contents into the string
    Ok(contents)
}
