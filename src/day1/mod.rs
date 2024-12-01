/**
 * Day 1 - Exercise 1 and 2
 * Advent of Code 2024
 * 
 * @Author: Michele
 */

use std::fs;


/**
 * Read the file and parse the values into two lists (left and right)
 * 
 * Returns a tuple with the two lists
 */
fn read_and_parse() -> (Vec<i32>, Vec<i32>){
    let path = "./src/day1/input/1.txt";
    

    let file_text = fs::read_to_string(path).unwrap();

    let mut left : Vec<i32> = vec![];
    let mut right : Vec<i32> = vec![];

    let lines = file_text.lines();
    for line in lines {
        let parts : Vec<String> = line.split("   ").map(str::to_string).collect();

        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            continue;
        }else{
            let left_val = parts[0].parse::<i32>().unwrap();
            let right_val = parts[1].parse::<i32>().unwrap();
            left.push(left_val);
            right.push(right_val);
        }

        left.sort();
        right.sort();
    }

    return (left, right);
}

/**
 * Calculate the total distance between two lists 
 * (absolute difference between same index values)
 */
pub fn run_ex1(){

    let mut sum: i32 = 0;

    let (left, right) = read_and_parse();

    for i in 0..left.len(){
        sum += (left[i] - right[i]).abs();
    }

    println!("Total Distance: {}", sum);
}

/**
 * Count the number of times a value appears in a ordered list
 */
fn count_repetitions(val: &i32, list: &Vec<i32>) -> i32{
    let mut count: i32 = 0;
    for i in list.iter(){
        if i == val{
            count += 1;
        }
        else if i > val {
            break;
        }
    }
    return count;
}

/**
 * Calculate the similarity score between two lists
 * (sum of the product of the number of times a value appears in the left list and the value itself)
 */
pub fn run_ex2(){
    let mut sum: i32 = 0;

    let (left, right) = read_and_parse();
    
    for i in left.iter(){
        let count = count_repetitions(i, &right);

        sum += count * i;
    }

    println!("Similarity Score: {}", sum);
}


/**
 * Run both exercises
 */
pub fn run(){
    run_ex1();
    run_ex2();
}