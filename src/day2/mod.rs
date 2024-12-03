/**
 * Day 2 - Exercise 1 and 2
 * Advent of Code 2024
 *
 * @Author: Michele
 */
use std::fs;

/**
 * Read the file and parse data into a matrix
 *
 * rows are reports, columns are levels
 */
fn read_and_parse() -> Vec<Vec<i32>> {
    let path: &str = "./src/day2/input.txt";

    let file_text: String = fs::read_to_string(path).unwrap();

    let mut reports: Vec<Vec<i32>> = vec![];

    let lines: std::str::Lines<'_> = file_text.lines();

    for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|s: &str| s.parse().unwrap()).collect();

        reports.push(levels);
    }
    
    println!("Report count: {}", reports.len());

    return reports;
}

/**
 * Check if a report is safe
 */
fn check_report(report : Vec<i32>) -> bool {

    if report.len() < 2 {
        println!("Invalid report: {:?}", report);
        return false;
    }

    let first_diff: i32 = report[1] - report[0]; 

    if first_diff.abs() > 0 && first_diff.abs() < 4 {

        let direction: i32 = first_diff.signum();

        for i in 2..report.len() {
            let diff: i32 = report[i] - report[i - 1];

            if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != direction {
                return false;
            }
        }

        return true;

    }

    return false;
}

/**
 * Check safety of report:
 * - report must be decrescent or crescent (not both)
 * - report must have a difference of at least 1 and at most 3 between levels
 *
 * counts the number of valid reports
 */
pub fn run_ex1() {
    let reports: Vec<Vec<i32>> = read_and_parse();

    let mut valid_reports: i32 = 0;

    for report in reports {
        if check_report(report) {
            valid_reports += 1;
        }
    }

    println!("Valid Reports: {}", valid_reports);
}

/**
 * Check safety with problem dampener:
 * if a report is unsafe, you can remove one level from it and check again
 * 
 * brute force approach
 */
pub fn run_ex2() {
    let reports: Vec<Vec<i32>> = read_and_parse();

    let mut valid_reports: i32 = 0;

    for report in reports {
        if check_report(report.clone()) {
            valid_reports += 1;
        } else {
            let mut new_report: Vec<i32> = report.clone();

            for i in 0..report.len() {
                new_report.remove(i);

                if check_report(new_report) {
                    valid_reports += 1;
                    break;
                }
                
                new_report = report.clone();
            }
        }
    }

    println!("Valid Reports with Dampener: {}", valid_reports);
}

pub fn run() {
    run_ex1();
    run_ex2();
}
