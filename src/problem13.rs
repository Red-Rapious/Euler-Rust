/*
Problem 13:
Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
*/

use std::fs;

pub fn problem13() -> u128 {
    let digits_to_keep = 13;

    // Load the numbers, while keeping only the first 13 digits of each
    let numbers = get_numbers(digits_to_keep);
    let mut s = 0;
    for n in numbers {
        s += n;
    }

    // Show only the first 10 digits of the sum
    while s >= 10_000_000_000 {
        s /= 10
    }
    s
}

fn get_numbers(digits_to_keep: u8) -> [u128; 100] {
    let file_path = "problem13.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file.");
    let iterator = contents.split("\n");

    let mut numbers = [0; 100];
    let mut i = 0;
    for number in iterator {
        let digits: Vec<u128> = number.chars().map(|c| c as u128 - '0' as u128).collect();
        for d in 0..digits_to_keep {
            numbers[i] *= 10;
            numbers[i] += digits[d as usize]
        }
        i += 1;
    }
    numbers
}