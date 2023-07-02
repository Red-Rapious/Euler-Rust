/*
Problem 9:
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2.
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

pub fn problem9() -> i32 {
    for a in 1..998 {
        for b in 1..std::cmp::min(a, 999 - a) {
            let c = 1_000 - a - b;
            if a*a + b*b == c*c {
                return a*b*c
            }
        }
    }
    // This should never happen according to the problem
    -1
}