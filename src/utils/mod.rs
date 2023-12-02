use std::env;
use std::fs;
use std::collections::HashMap;
#[allow(dead_code)]
pub fn read_input(path: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("src").join(path).join("input.txt");
    let contents = fs::read_to_string(&file_path)
        .expect("Verify that an input.txt is present");
    contents
}
#[allow(dead_code)]
pub fn lst_to_hashmap<'a, K, V>(data: &'a [(K, V)]) -> HashMap<&'a K, &'a V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone + Eq,
{
    data.iter().map(|(k, v)| (k, v)).collect()
}