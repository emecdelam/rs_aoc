use crate::utils;
use std::collections::HashMap;
pub fn main() {
    let contents = utils::read_input("_2023\\q1");
    let splitted = contents.split("\n");
    let mut sum = 0;
    for i in splitted.clone() {
        let mut first:char = '0';
        let mut second :char = '0';
        let chars = i.chars();
        for c in chars {
            if c.is_numeric(){
                first = c;
                break;
            }
        }
        for c in i.chars().rev() {
            if c.is_numeric() {
                second = c;
                break;
            }
        }
        let res  = format!("{}{}", first, second);
        let f_res :i32 = res.parse().unwrap();
        sum += f_res;
    }
    println!("First part sum : {}",sum);
    let converter: HashMap<&str, &str> = [("zero", "0"),("one", "1"),("two", "2"),("three", "3"),("four", "4"),("five", "5"),("six", "6"),("seven", "7"),("eight", "8"),("nine", "9"),]
        .iter()
        .cloned()
        .collect();
    sum = 0;

    for i in splitted {
        let chars = i.chars();
        let mut checker1 = String::new();
        let mut checker2 = String::new();
        let mut first: char = '0';
        let mut second = '0';
        for c in chars {
            checker1.push(c);
            if let Some(value) = convert_in_map(&converter, &checker1) {
                first = value.chars().next().unwrap();
                break;
            }
            if c.is_numeric() {
                first = c.clone();
                break;
            }
        }
        for c in i.chars().rev() {
            checker2.insert_str(0,c.to_string().as_str());
            if let Some(value) = convert_in_map(&converter, &checker2) {
                second = value.chars().next().unwrap();
                break;
            }
            if c.is_numeric() {
                second = c;
                break;
            }
        }
        let res  = format!("{}{}", first, second);
        let f_res :i32 = res.parse().unwrap();
        sum += f_res;
    }
    println!("Second part sum : {}",sum);
}

fn convert_in_map<'a>(map: &'a HashMap<&str, &str>, string: &str) -> Option<&'a str> {
    for key in map.keys() {
        if string.contains(key) {
            return Some(map[key]);
        }
    }
    None
}
