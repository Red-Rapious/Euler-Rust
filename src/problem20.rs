/*
Problem 20:
n! means n × (n -1) x... ×3 × 2 × 1.
For example, 10! = 10 × 9 x ... × 3 x 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8+0+0 = 27.

Find the sum of the digits in the number 100!.
*/

use num_bigint::BigUint;

pub fn problem20(n: u32) -> u128 {
    // Compute n!
    let mut fact = BigUint::from_radix_be(&(vec![1]), 10).unwrap();
    for i in 2..n+1 {
        fact *= i;
    }
    let base10 = fact.to_radix_be(10);

    // Sum the digits in base 10
    let mut s: u128 = 0;
    for number in base10 {
        s += number as u128;
    }
    s
}