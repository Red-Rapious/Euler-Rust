/*
Problem 10:
The sum of the primes below 10 is 2+3+5+7 = 17.
Find the sum of all the primes below two million.
*/

fn is_prime(n: i32) -> bool {
    let mut prime = true;
    for i in 2..((n as f32).sqrt() as i32 + 1) {
        if n % i == 0 {
            prime = false;
            break;
        }
    }
    prime
}

pub fn problem10() -> i128 {
    // This is a simple approach checking every number.
    let mut s: i128 = 0;

    for n in 2..2_000_001 {
        if is_prime(n) { s += n as i128; }
    }
    s
}