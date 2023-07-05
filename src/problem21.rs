/*
Problem 21:
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a + b, then a and b are an amicable pair and each of a and b are called amicable numbers.
For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; 
therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

fn d(n: usize) -> usize {
    let mut s = 0;
    for i in 1..n {
        if n % i == 0 { s+= i; }
    }
    s
}

pub fn problem21() -> usize {
    let mut s = 0;

    for i in 1..10_000 {
        let di = d(i);

        if di < i { // avoids the case where i == d(i), and avoids counting & computing each pair twice
            let ddi = d(di);
            if ddi == i { 
                s += i;
                s += di;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem21() {
        assert_eq!(problem21(), 31_626);
    }
}