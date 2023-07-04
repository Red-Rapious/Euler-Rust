use std::io;
use std::env;

use crate::problem1::problem1;
use crate::problem1::problem1_v2;

use crate::problem2::problem2;
use crate::problem3::problem3;
use crate::problem4::problem4;
use crate::problem5::problem5;
use crate::problem5::problem5_v2;
use crate::problem6::problem6;
use crate::problem6::problem6_v2;
use crate::problem7::problem7;
use crate::problem8::problem8;
use crate::problem9::problem9;
use crate::problem10::problem10;
use crate::problem11::problem11;
use crate::problem12::problem12;
use crate::problem13::problem13;
use crate::problem14::problem14;
use crate::problem15::problem15;

pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem9;
pub mod problem10;
pub mod problem11;
pub mod problem12;
pub mod problem13;
pub mod problem14;
pub mod problem15;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut problem_number = String::new();

    if args.len() == 1 {
        println!("Problem number:");

        // Read problem number input
        io::stdin()
             .read_line(&mut problem_number)
             .expect("Failed to read line");
    }
    else if args.len() == 2 {
        problem_number = args[1].clone();
    }
    else {
        println!("Too many arguments. The length of 'args' is {}, whereas it should be 2.", args.len());
        std::process::exit(0);
    }
    

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
    else if problem_number == 5 {
        let value2 = problem5_v2();
        println!("Problem 5 solution: {value2} (v2)");
        let value1 = problem5();
        println!("Problem 5 solution: {value1} (v1)");
    }
    else if problem_number == 6 {
        let value2 = problem6_v2(100);
        println!("Problem 6 solution: {value2} (v2)");
        let value1 = problem6(100);
        println!("Problem 6 solution: {value1} (v1)");
    }
    else if problem_number == 7 {
        let value = problem7();
        println!("Problem 7 solution: {value}");
    }
    else if problem_number == 8 {
        let value = problem8();
        println!("Problem 8 solution: {value}");
    }
    else if problem_number == 9 {
        let value = problem9();
        println!("Problem 9 solution: {value}");
    }
    else if problem_number == 10 {
        let value = problem10();
        println!("Problem 10 solution: {value}");
    }
    else if problem_number == 11 {
        let value = problem11();
        println!("Problem 11 solution: {value}");
    }
    else if problem_number == 12 {
        let value = problem12();
        println!("Problem 12 solution: {value}");
    }
    else if problem_number == 13 {
        let value = problem13();
        println!("Problem 13 solution: {value}");
    }
    else if problem_number == 14 {
        println!("Problem 14 solution: {}", problem14());
    }
    else if problem_number == 15 {
        println!("Problem 15 solution: {}", problem15());
    }
    else {
        println!("Incorrect problem number. End of program.");
    }
}