/*
Problem 7:
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10001st prime number?
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

pub fn problem7() -> i32 {
    let mut count = 1;
    let mut n = 1;

    while count != 10_001 {
        n += 2;
        if is_prime(n) { count += 1; }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem7() {
        assert_eq!(problem7(), 104_743);
    }
}