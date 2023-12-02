use std::collections::HashMap;

use crate::utils;

pub fn main() {
    let contents = utils::read_input("_2023\\q2");
    let parts:Vec<&str> = contents.split("\n").collect();
    let mut sum = 0;
    for (index, part) in parts.clone().iter().enumerate() {
        let data: Vec<&str> = part.split(":").collect();
        let parts = data[1].split(";");
        let mut is_valid :bool = true;
        for part in parts{
            let game_info = parse_game_info(part);
            if valider(game_info) {
                is_valid = false;
                break;
            }
        }
        if is_valid{
            sum += index+1;
        }
    }
    println!("First part {}",sum);
    let mut sum2 = 0;
    for (_index, part) in parts.iter().enumerate() {
        let data: Vec<&str> = part.split(":").collect();
        let parts = data[1].split(";");
        let mut min_red:u32 = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for part in parts{
            let game_info = parse_game_info(part);
            if let Some(&blue_quantity) = game_info.get("blue") {
                min_blue = comparator(min_blue, blue_quantity);
            }
            if let Some(&red_quantity) = game_info.get("red"){
                min_red = comparator(min_red, red_quantity);
            }
            if let Some(&green_quantity) = game_info.get("green"){
                min_green = comparator(min_green, green_quantity)
            }
        }
        sum2 += min_blue * min_green * min_red
        
    }
    println!("Second part {}",sum2)
}

fn parse_game_info(game_info: &str) -> HashMap<&str, u32> {
    let mut result = HashMap::new();
    for part in game_info.split(',').map(|s| s.trim()) {
        let mut iter = part.split_whitespace();
        if let (Some(quantity_str), Some(color)) = (iter.next(), iter.next()) {
            if let Ok(quantity) = quantity_str.parse::<u32>() {
                result.insert(color, quantity);
            }
        }
    }

    result
}
fn comparator(min: u32, input: u32) -> u32 {
    if input > min {
        return input;
    }
    return min;
}
fn valider(map : HashMap<&str,u32>) -> bool{
    if let Some(&blue_quantity) = map.get("blue") {
        if blue_quantity > 14 {
            return true;
        }
    }
    if let Some(&red_quantity) = map.get("red"){
        if red_quantity > 12 {
            return true;
        }
    }
    if let Some(&green_quantity) = map.get("green"){
        if green_quantity > 13 {
            return true;
        }
    }
    return false;
}