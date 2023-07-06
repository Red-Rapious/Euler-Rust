/*
Problem 23:
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, 
the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,  which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n 
and it is called abundant if this sum exceeds n.
As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two
abundant numbers is 24. 
By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. 
However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number 
that cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
*/

use std::collections::HashSet;

fn d(n: usize) -> usize {
    let mut s = 0;
    for i in 1..n {
        if n % i == 0 { s+= i; }
    }
    s
}

pub fn problem23() -> usize {
    // Create a set containing all numbers below 28_124
    let mut set = HashSet::new();
    for i in 1..28_124 {
        set.insert(i);
    }

    // Compute every abundant number
    let mut abundant_numbers = vec![];
    for i in 1..28_124 {
        if d(i) > i {
            abundant_numbers.push(i);
        }
    }

    // Remove the sums of all abundant numbers
    for i in &abundant_numbers {
        for j in &abundant_numbers {
            if j > i { break; }
            if set.contains(&(i+j)) { set.remove(&(i+j)); }
        }
    }

    // Return the sum of remaining integers
    set.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem23() {
        assert_eq!(problem23(), 4_179_871);
    }
}