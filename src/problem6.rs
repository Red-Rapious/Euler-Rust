/* 
The sum of the squares of the first ten natural numbers is 1^2 + 2^2 + ... + 10^2 = 385.
The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)^2 = 55^2 = 3025.
Hence the difference between the sum of the squares of the first ten natural numbers 
and the square of the sum is 3025 - 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum. 
*/

pub fn problem6(n: i64) -> i64 {
    let mut s1: i128 = 0;
    let mut s2: i128 = 0;

    for i in 1..((n as i128) +1) {
        s1 += i*i;
        s2 += i;
    }
    ((s2*s2) - s1) as i64
}

pub fn problem6_v2(n: i64) -> i64 {
    let s1 = n*(n+1)*((2*n)+1)/6;
    let s2 = n*(n+1)/2;
    s2*s2 - s1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem6() {
        assert_eq!(problem6(100), 25_164_150);
    }

    #[test]
    fn test_problem6_v2() {
        assert_eq!(problem6_v2(100), 25_164_150);
    }
}