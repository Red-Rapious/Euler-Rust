use std::io;

use crate::problem1::problem1;
use crate::problem1::problem1_v2;

use crate::problem2::problem2;
use crate::problem3::problem3;
use crate::problem4::problem4;

pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;

fn main() {
    let mut problem_number = String::new();

    println!("Problem number:");

    // Read problem number input
    io::stdin()
         .read_line(&mut problem_number)
         .expect("Failed to read line");

    // Convert, if possible, the input to int32
    let problem_number: i32 = problem_number.trim().parse().expect("Please type a number!");

    if problem_number == 1 {
        let value = problem1();
        let value2 = problem1_v2();
        println!("Problem 1 solution: {value} (v1), {value2} (v2)");
    }
    else if problem_number == 2 {
        let value = problem2();
        println!("Problem 2 solution: {value}");
    }
    else if problem_number == 3 {
        let value = problem3(600_851_475_143);
        println!("Problem 3 solution: {value}");
    }
    else if problem_number == 4 {
        let value = problem4();
        println!("Problem 4 solution: {value}");
    }
    else {
        println!("Incorrect problem number. End of program.");
    }
}