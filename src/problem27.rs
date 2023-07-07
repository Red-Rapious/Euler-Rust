/*
Problem 27:
Euler discovered the remarkable quadratic formula:
n^2 + n + 41
It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n < 39. However, when
n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 412 + 41 + 41 is clearly divisible
by 41.

The incredible formula n^2 - 79 + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n < 79.
The product of the coefficients, -79 and 1601, is -126479.
Considering quadratics of the form:
n^2 + an + 6, where |a| < 1000 and |b| <= 1000
where n is the modulus/absolute value of n, e.g. |11| = 11 and |-4| = 4

Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for
consecutive values of n, starting with n = 0.
*/

fn is_prime(n: i32) -> bool {
    if n <= 1 { return false; }

    let mut prime = true;
    for i in 2..((n as f32).sqrt() as i32 + 1) {
        if n % i == 0 {
            prime = false;
            break;
        }
    }
    prime
}

pub fn problem27() -> i32 {
    let mut max = (0, 0);
    for a in -999..1000 {
        for b in -1000..1000 {
            let mut n = 0;
            while is_prime(n*n + a*n + b) {
                n += 1;
            }

            if n > max.0 {
                max = (n, a*b)
            }
        }
    }
    max.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem27() {
        assert_eq!(problem27(), -59_231);
    }
}