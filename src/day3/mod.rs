/**
 * Day 3 - Exercise 1 and 2
 * Advent of Code 2024
 *
 * @Author: Michele
 */

use std::fs;
use regex::Regex;

const PATH: &str = "./src/day3/input.txt";

const EXPR: &str = r"mul\((\d+),(\d+)\)";

/**
 * Find all the code mul(num1, num2) and sum the result of num1 * num2
 */
pub fn sum_multiply(file_text: &str) -> i32 {
    let reg: Regex = Regex::new(EXPR).unwrap();

    let captures: regex::CaptureMatches<'_, '_> = reg.captures_iter(&file_text);

    let mut sum : i32 = 0;

    for cap in captures {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();

        sum += num1 * num2;
    }

    return  sum;
}

/**
 * Find all the code mul(num1, num2) and sum the result of num1 * num2
 */
pub fn run_ex1() {
    let file_text: String = fs::read_to_string(PATH).unwrap();

    let sum: i32 = sum_multiply(&file_text);

    println!("Sum 1: {}", sum);
}


/**
 * Ignore all the mul(num1, num2) that are between a don't() and do() block
 */
pub fn run_ex2() {

    let mut file_text: String = fs::read_to_string(PATH).unwrap();

    // Newlines makes regex go crazy
    file_text = file_text.replace("\n", "");


    let remove: Regex = Regex::new(r"don't\(\).*?(?:do\(\))").unwrap();

    let mut clean_text: String = remove.replace_all(&file_text, "").to_string();

    // Rust regex doesn't support lookbehind, so I have to split the string in case there is a don't() that is not closed
    // With other languages you can add $ to the regex to check if the line ends
    // "don't\(\).*?(?:do\(\)|$)"
    let last_dont: Option<usize>  = clean_text.find("don't()");

    if last_dont.is_some() {
        println!("Found a don't() that is not closed (index: {})", last_dont.unwrap());
        clean_text = clean_text.split_at(last_dont.unwrap()).0.to_string();
        
    }

        
    let sum: i32 = sum_multiply(&clean_text);

    println!("Sum 2: {}", sum);
    

}

pub fn run() {
    run_ex1();
    run_ex2();
}