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
use crate::problem16::problem16;
use crate::problem17::problem17;
use crate::problem18::problem18;
use crate::problem19::problem19;
use crate::problem20::problem20;
use crate::problem21::problem21;
use crate::problem22::problem22;
use crate::problem23::problem23;
use crate::problem24::problem24;
use crate::problem24::problem24_v2;
use crate::problem25::problem25;
//use crate::problem26::problem26;
use crate::problem27::problem27;
use crate::problem28::problem28;
use crate::problem29::problem29;
use crate::problem30::problem30;

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
pub mod problem16;
pub mod problem17;
pub mod problem18;
pub mod problem19;
pub mod problem20;
pub mod problem21;
pub mod problem22;
pub mod problem23;
pub mod problem24;
pub mod problem25;
pub mod problem26;
pub mod problem27;
pub mod problem28;
pub mod problem29;
pub mod problem30;

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
        println!("Problem 1 solution: {} (v1), {} (v2)", problem1(), problem1_v2());
    }
    else if problem_number == 2 {
        println!("Problem 2 solution: {}", problem2());
    }
    else if problem_number == 3 {
        println!("Problem 3 solution: {}", problem3(600_851_475_143));
    }
    else if problem_number == 4 {
        println!("Problem 4 solution: {}", problem4());
    }
    else if problem_number == 5 {
        println!("Problem 5 solution: {} (v2)", problem5_v2());
        println!("Problem 5 solution: {} (v1)", problem5());
    }
    else if problem_number == 6 {
        println!("Problem 6 solution: {} (v2)", problem6_v2(100));
        println!("Problem 6 solution: {} (v1)", problem6(100));
    }
    else if problem_number == 7 {
        println!("Problem 7 solution: {}", problem7());
    }
    else if problem_number == 8 {
        println!("Problem 8 solution: {}", problem8());
    }
    else if problem_number == 9 {
        println!("Problem 9 solution: {}", problem9());
    }
    else if problem_number == 10 {
        println!("Problem 10 solution: {}", problem10());
    }
    else if problem_number == 11 {
        println!("Problem 11 solution: {}", problem11());
    }
    else if problem_number == 12 {
        println!("Problem 12 solution: {}", problem12());
    }
    else if problem_number == 13 {
        println!("Problem 13 solution: {}", problem13());
    }
    else if problem_number == 14 {
        println!("Problem 14 solution: {}", problem14());
    }
    else if problem_number == 15 {
        println!("Problem 15 solution: {}", problem15());
    }
    else if problem_number == 16 {
        println!("Problem 16 solution: {}", problem16());
    }
    else if problem_number == 17 {
        println!("Problem 17 solution: {}", problem17());
    }
    else if problem_number == 18 {
        println!("Problem 18 solution: {}", problem18());
    }
    else if problem_number == 19 {
        println!("Problem 19 solution: {}", problem19());
    }
    else if problem_number == 20 {
        println!("Problem 20 solution: {}", problem20(100));
    }
    else if problem_number == 21 {
        println!("Problem 21 solution: {}", problem21());
    }
    else if problem_number == 22 {
        println!("Problem 22 solution: {}", problem22());
    }
    else if problem_number == 23 {
        println!("Problem 23 solution: {}", problem23());
    }
    else if problem_number == 24 {
        println!("Problem 24 solution: {} (v1)", problem24(1_000_000));
        println!("Problem 24 solution: {} (v2: Permutations crate)", problem24_v2(1_000_000));
    }
    else if problem_number == 25 {
        println!("Problem 25 solution: {}", problem25());
    }
    /*else if problem_number == 26 {
        println!("Problem 26 solution: {}", problem26());
    }*/
    else if problem_number == 27 {
        println!("Problem 27 solution: {}", problem27());
    }
    else if problem_number == 28 {
        println!("Problem 28 solution: {}", problem28(1_001));
    }
    else if problem_number == 29 {
        println!("Problem 29 solution: {}", problem29(100));
    }
    else if problem_number == 30 {
        println!("Problem 30 solution: {}", problem30(5));
    }
    else {
        println!("Incorrect problem number. End of program.");
    }
}