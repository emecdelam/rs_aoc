use crate::utils;
use std::collections::VecDeque;
use std::collections::HashMap;
pub fn main() {
    let contents = utils::read_input("_2023\\q4");
    let lines:Vec<&str> = contents.split("\n").collect();
    let mut sum:i32 = 0;
    let mut counts:HashMap<i32, i32> = HashMap::new();
    let mut game_index:i32=0;
    for line in lines{
        let mut count:i32 = 0;
        let data: Vec<&str> = line.split(":").collect();
        game_index+=1;
        let mut winning:VecDeque<i32> = VecDeque::new();
        let mut input:VecDeque<i32> = VecDeque::new();
        let mut current:String = String::new();
        let mut not_winning = true;
        for c in data[1].chars(){
            if not_winning{
                if c.is_numeric(){
                    current.push(c);
                } else {
                    if !current.is_empty(){
                        let parsed:i32 = current.as_str().parse().unwrap();
                        winning.push_back(parsed);
                        current = String::new();
                    }

                }
            } else {
                if c.is_numeric(){
                    current.push(c);
                } else {
                    if !current.is_empty(){
                        let parsed:i32 = current.parse().unwrap();
                        input.push_back(parsed);
                        current = String::new();
                    }

                }
            }
            if c == '|'{
                not_winning = false;
                current = String::new();
            }
        }

        for num in input{
            for win in &winning{
                if num == *win{
                    count+=1;
                }
            }
        }
        if count >= 2{
            sum += 2_i32.pow((count-1) as u32); 
        } else if count == 1 {
            sum+=1;
        } else {
            sum+=0;
        }
        if !counts.contains_key(&game_index){
            counts.insert(game_index, 1);
        }
        else if counts.contains_key(&game_index){
            let entry = counts.entry(game_index);
            match entry {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() += 1;
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(1);
                }
            }
        }
        for i in game_index+1..count+game_index+1{

            if !counts.contains_key(&(i)){
                if let Some(&value) = counts.get(&game_index) {
                    counts.insert(i, value);
                } else {
                    println!("Key {} not found in the HashMap.", game_index);
                }
            } 
            else if counts.contains_key(&(i)){
                if let Some(&value) = counts.get(&game_index) {
                    let entry = counts.entry(i);
                    match entry {
                        std::collections::hash_map::Entry::Occupied(mut occupied) => {
                            *occupied.get_mut() += value;
                        }
                        std::collections::hash_map::Entry::Vacant(vacant) => {
                            vacant.insert(1);
                        }
                    }
                } else {
                    println!("Key {} not found in the HashMap.", game_index);
                }

            }
        }
    }
    println!("Sum : {}",sum);
    println!("Sum : {}",count_hashmap(&counts));

}
fn print_hashmap(hashmap: &HashMap<i32, i32>) {
    for (key, value) in hashmap {
        print!("Key: {}, Value: {} ; ", key, value);
    }
    println!()
}
fn count_hashmap(hashmap: &HashMap<i32, i32>) -> i32{
    let mut sum:i32 = 0;
    for value in hashmap{
        sum+= value.1;
    }
    return sum;
}