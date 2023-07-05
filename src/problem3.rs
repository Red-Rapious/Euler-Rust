/*
Problem 3:
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
*/

pub fn problem3(input: i64) -> i64 {
    let mut current: i64 = input;
    let mut i: i64 = 2;
    let mut max_factor: i64 = 1;

    loop {
        if i > current { break; }
        if current % i == 0 {
            current = current / i;
            max_factor = i;
        }
        i += 1;
    }
    max_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem3() {
        assert_eq!(problem3(600_851_475_143), 6_857);
    }
}