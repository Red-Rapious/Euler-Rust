/*
Problem 24:
A permutation is an ordered arrangement of objects. 
For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. 
The lexicographic permutations of 0, 1 and 2 are: 012 021 102 120 201 210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/

use permutations::Permutations;

fn fact(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

pub fn problem24(nth: usize) -> usize {
    let mut goal = nth-1;
    let mut remaining_digits: Vec<usize> = (0..10).collect();
    let mut chosen_digits: Vec<usize> = vec![];

    while remaining_digits.len() != 0 {
        let n = remaining_digits.len();
        let f = fact(n-1);
        
        let i = goal / f;
        goal %= f;
        let digit = remaining_digits[i];
        remaining_digits.remove(i);

        chosen_digits.push(digit);
    }

    let mut s = 0;
    for i in 0..chosen_digits.len() {
        s += chosen_digits[i] * (10 as usize).pow(9-i as u32);
    }
    s
}

pub fn problem24_v2(nth: usize) -> usize {
    // This implementation uses the crate 'Permutations'

    // Generate all permutations of [|0, 9|]
    let permutations = Permutations::new(10);
    // Select the n-th in lexicographic order
    let perm = permutations.get(nth-1).unwrap();

    // Convert it to a vec of digits with the proper power of 10
    let digits = (0..10).map(|i| perm.apply(i)*(10 as usize).pow(9-i as u32));
    digits.sum() // Get the final number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem24() {
        assert_eq!(problem24(1_000_000), 2783915460);
    }

    #[test]
    fn test_problem24_v2() {
        assert_eq!(problem24_v2(1_000_000), 2783915460);
    }
}