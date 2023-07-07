/*
Problem 30:
Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
1634 = 1^4 + 6^4 + 3^4 + 4^4
8208 = 8^4 + 2^4 + 0^4 + 8^4
9474 = 9^4 + 4^4 + 7^4 + 4^4
As 1 = 1^4 is not a sum it is not included.
The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
*/

pub fn problem30(power: u32) -> usize {
    let mut total = 0;

    // One can prove that there is a maximum upper bound, so we'll only check numbers with 6 digits or less
    for n in 2..1_000_000 {
        let mut sum = 0;
        let mut current = n;

        while current > 0 {
            sum += (current % 10 as usize).pow(power);
            current /= 10;
        }

        if sum == n { total += n; }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem30() {
        assert_eq!(problem30(4), 19_316);
        assert_eq!(problem30(5), 443_839);
    }
}