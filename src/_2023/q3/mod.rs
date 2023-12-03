use crate::utils;
pub fn main() {
    let contents = utils::read_input("_2023\\q3");
    let matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut boolean_matrix: Vec<Vec<bool>> = vec![vec![false; matrix[0].len()]; matrix.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_numeric(){
                if check_direction(&matrix, j, i){
                    boolean_matrix[i][j] = true;
                }
            }
            if boolean_matrix[i][j]{
                let mut count:usize = 0;
                let mut can_continue:bool = true;
                while can_continue {
                    count+=1;
                    if (j+count) >= matrix[i].len(){
                        can_continue = false;
                    }
                    if can_continue{
                        if matrix[i][j+count].is_numeric(){
                            boolean_matrix[i][j+count] = true;
                        } else {
                            can_continue = false;
                        }
                    }
                }
                can_continue = true;
                count = 0;
                while can_continue {
                    count+=1;
                    if j == count-1{
                        can_continue = false;
                    }
                    if can_continue{
                        if matrix[i][j-count].is_numeric(){
                            boolean_matrix[i][j-count] = true;
                        } else {
                            can_continue = false;
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for i in 0..boolean_matrix.len(){
        let mut count = 0;
        let mut stop = false;
        let mut result_string = String::new();
        while count<boolean_matrix[i].len(){
            if boolean_matrix[i][count]{
                stop = true;
            } 
            if !boolean_matrix[i][count]{
                if !result_string.is_empty(){
                    sum+= result_string.as_str().parse::<i32>().unwrap();
                    result_string = String::new();
                }
                stop = false;
                
            }
            if stop{
                result_string.push(matrix[i][count]);
            }
            count+=1;
        }
        if !result_string.is_empty(){
            sum+= result_string.as_str().parse::<i32>().unwrap();
        }
    }
    println!("Sum : {}",sum);
}

fn check_direction(matrix: &Vec<Vec<char>>, index: usize, line:usize) -> bool {
    let mut valid = false;
    let coor = (line,index);
    if coor.1 <  matrix[line].len()-1{
        is_special(matrix[line][index+1], &mut valid);
    }
    if coor.1 > 0 {
        is_special(matrix[line][index-1], &mut valid);
    }
    if coor.0 < matrix.len() -1{
        is_special(matrix[line+1][index], &mut valid);
        if coor.1 <  matrix[line].len()-1{
            is_special(matrix[line+1][index+1], &mut valid);
        }
        if coor.1 > 0 {
            is_special(matrix[line+1][index-1], &mut valid);
        }
    }
    if coor.0 > 0{
        is_special(matrix[line-1][index], &mut valid);
        if coor.1 <  matrix[line].len()-1{
            is_special(matrix[line-1][index+1], &mut valid);
        }
        if coor.1 > 0 {
        is_special(matrix[line-1][index-1], &mut valid);
        }
    }
    valid
}
fn is_special(input: char, valid: &mut bool) {
    if !*valid {
        if input.is_numeric() || input == '.' {
            return;
        }
        *valid = true;
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn print_boolean_matrix(matrix: &Vec<Vec<bool>>) {
    for row in matrix {
        for &value in row {
            print!("{}", if value { '1' } else { '0' });
        }
        println!();
    }
    println!();
}