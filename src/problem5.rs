/*
Problem 5:
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

pub fn problem5() -> i32 {
    let mut n = 1;

    // This is a bruteforce approach checking every number and every i in [1, 20]
    // A mathematical approach would be to take each prime factor with the highest multiplicity in [1, 20]
    loop {
        let mut divisible = true;
        for i in 1..21 {
            if n % i != 0 {
                divisible = false;
                break;
            }
        }

        if divisible {
            return n;
        }
        n += 1;
    }
}

pub fn problem5_v2() -> i32 {
    // Primes under 20. Can be generated using a script.
    let mut primes = vec![(2, 1), (3, 1), (5, 1), (7, 1), (11, 1), (13, 1), (17, 1), (19, 1)];
    
    for i in 1..21 {
        for j in 0..(primes.len()-1) {
            let mut q = i;
            let mut multi = 0;
            while q % primes[j].0 == 0 {
                q = q/primes[j].0;
                multi += 1;
            }
            if multi > primes[j].1 {
                primes[j].1 = multi
            }
        }
    }
    let mut prod = 1;
    for (p, m) in primes {
        prod *= (p as i32).pow(m);
    }
    prod as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem5() {
        assert_eq!(problem5_v2(), 232_792_560);
    }
}