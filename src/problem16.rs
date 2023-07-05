/*
Problem 16:
2^15 = 32_768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2^1000?
*/

use num_bigint::BigUint;

pub fn problem16() -> u32 {
    // The main difficulty of this problem is that 2^1000 is a very large number.
    // Therefore, I decided to store it in a BigUint, which is basically a Vector of u8
    
    // Create the number in base 2
    let mut base2 = vec![];
    base2.push(1);
    for _ in 0..1_000 {
        base2.push(0);
    }

    // Convert it in base 10
    let number = BigUint::from_radix_be(&base2, 2).unwrap();
    let base10 = number.to_radix_be(10);

    // Take the sum
    let mut s = 0;
    for n in base10 {
        s += n as u32;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem16() {
        assert_eq!(problem16(), 1_366);
    }
}