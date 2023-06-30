use std::io;

use crate::problem1::problem1;
use crate::problem1::problem1_v2;
pub mod problem1;

fn main() {
    let mut problem_number = String::new();

    println!("Problem number:");
    io::stdin()
         .read_line(&mut problem_number)
         .expect("Failed to read line");

    let problem_number: i32 = problem_number.trim().parse().expect("Please type a number!");

    if problem_number == 1 {
        let value = problem1();
        let value2 = problem1_v2();
        println!("Problem 1 solution: {value} (v1), {value2} (v2)");
    }
    else {
        println!("Incorrect problem number. End of program.");
    }
}