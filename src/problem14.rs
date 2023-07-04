/*
Problem 14:
The following iterative sequence is defined for the set of positive integers:
    n -> n/2 (n is even)
    n -> 3n + 1 (n is odd)
Using the rule above and starting with 13, we generate the following sequence: 13 -> 40 -> 20 - 10 - 5 - 16 - 8 - 4 - 2 - 1.
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. 
Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?
NOTE: Once the chain starts the terms are allowed to go above one million.
*/

fn collatz_chain_length(n: i64, depth: i64) -> i64 {
    if n == 1 {
        return depth
    }
    if n % 2 == 0 {
        return collatz_chain_length(n/2, depth+1)
    }
    else {
        return collatz_chain_length(3*n+1, depth+1)
    }
}

pub fn problem14() -> i64 {
    // This is a naive approach. I was actually quite surprised that it works,
    // considering both the time complexity and the recursion limit.
    let mut maxi = (0, 1);
    for i in 2..1_000_000 {
        // Compute the length of the collatz chain
        let l = collatz_chain_length(i, 0);
        
        // Compare it with the current maximum
        if l > maxi.0 {
            maxi = (l, i);
        }
    }
    maxi.1
}