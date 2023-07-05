/*
Problem 1:
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

pub fn problem1() -> i32 {
    // Checks every number from 1 to 999

    let mut s: i32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            s += i
        }
    }
    s
}

pub fn problem1_v2() -> i32 {
    // Sums only multiples of 3 (in a loop with a step of 3) 
    // plus the multiples of 5 (those who aren't multiple of 3 already)

    let mut sum = 0;
    let mut i = 3;
    while i < 1000 {
        sum += i;
        i += 3;
    }

    i = 5;
    while i < 1000 {
        if i % 3 != 0 {
            sum += i
        }
        i += 5;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(problem1(), 2_331_68);
    }

    #[test]
    fn test_problem1_v2() {
        assert_eq!(problem1_v2(), 2_331_68);
    }
}