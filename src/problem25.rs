/*
Problem 25:
The Fibonacci sequence is defined by the recurrence relation:
F_n+1 = F_n-1 + F_n-2, where F_1 = 1 and F_2 = 1.
Hence the first 12 terms will be:
F_1 = 1
F_2 = 1
F_3 = 2
F_4 = 3
F_5 = 5
F_6 = 8
F_7 = 13
F_8 = 21
F_9 = 34
F_10 = 55
F_11 = 89
F_12 = 144
The 12th term, F_12, is the first term to contain three digits.
What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
*/

use num_bigint::BigUint;

pub fn problem25() -> usize {
    let mut first = BigUint::from(1 as usize);
    let mut second = BigUint::from(1 as usize);
    let mut i = 2;

    while second.to_radix_be(10).len() < 1_000 {
        let temp = first;
        first = second.clone();
        second = temp.clone() + first.clone();
        i += 1;
    }
    
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem24() {
        assert_eq!(problem25(), 4_782);
    }
}