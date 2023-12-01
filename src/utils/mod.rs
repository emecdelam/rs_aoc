use std::env;
use std::fs;

pub fn read_input(path: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("src").join(path).join("input.txt");
    let contents = fs::read_to_string(&file_path)
        .expect("Verify that an input.txt is present");
    contents
}
