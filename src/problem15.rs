/*
Problem 15:
Starting in the top left corner of a 2*2 grid, and only being able to move to the right and down, 
there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20*20 grid? 
*/

pub fn problem15() -> u128 {
    // The calculation gives us that the result is \sum_{k=0}^n (C_n^k) ^ 2 
    // The reason is, each path is the combination of two half-paths: 
    //      - one that goes from top-left to the center diagonal
    //      - one that goes from that point on the center diagonal to the bottom-right
    // For the k-th center diagonal point, there is C_n^k paths from top-left and C_n^k paths to bottom-right, giving the announced result

    let n = 20;

    let mut s = 0;
    let mut binomial = 1;
    for k in 0..n+1 {
        s += binomial*binomial; // add the binomial squared

        // update the binomial
        binomial *= n-k; 
        binomial /= k+1;
    }
    s
}

// This is an alternate version that is more computer-science-oriented but in fact impractical
static mut TOTAL: i128 = 0;
const GRID_SIZE: (u8, u8) = (20, 20);

pub fn problem15_enumerate() -> i128 {
    // Enumerates all paths starting at the top left
    fn correct_paths(position: (u8, u8), remaining_moves: u8)  {
        if position.0 > GRID_SIZE.0 || position.1 > GRID_SIZE.1 {
            ()
        } else if remaining_moves == 0 {
            if position == GRID_SIZE {
                unsafe {
                    TOTAL += 1
                }
            } else {
                ()
            }
        } else {
            correct_paths((position.0 + 1, position.1), remaining_moves - 1);
            correct_paths( (position.0, position.1 + 1), remaining_moves - 1);
        }
    }

    correct_paths( (0, 0), GRID_SIZE.0 * 2);
    unsafe {
        TOTAL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem15() {
        assert_eq!(problem15(), 137_846_528_820);
    }
}