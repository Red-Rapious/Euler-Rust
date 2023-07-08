/*
Problem 26:
A unit fraction contains 1 in the numerator. 
The decimal representation of the unit fractions with denominators 2 to 10 are given:
1/2 = 0.5
1/3 = 0.(3)
1/4 = 0.25
1/5 = 0.2
1/6 = 0.1(6)
1/7 = 0. (142857)
1/8 = 0.125
1/9 = 0.(1)
1/10 = 0.1
Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1 /d contains the longest recurring cycle in its decimal fraction part.
*/

fn cycle_length(d: usize) -> usize {
    let mut reminder = 10;
    let mut lambda = 0;

    while reminder != 10 || lambda < 1 {
        reminder = (reminder % d) * 10;
        lambda += 1;
    }

    lambda
}

pub fn problem26() -> usize {
    let mut max = (0, 0);
    for d in 2..1001 {
        if d % 2 != 0 && d % 5 != 0 {
            let cycle = cycle_length(d);
            if cycle > max.0 {
                max = (cycle, d)
            }
        }
    }
    max.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem26() {
        assert_eq!(problem26(), 983);
    }
}