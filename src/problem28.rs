/*
Problem 28:
Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
21 22 23 24 25
20  7  8  9 10 
19  6  1  2 11 
18  5  4  3 12
17 16 15 14 13
It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
*/

enum Direction {
    Down,
    Left,
    Up,
    Right,
}

pub fn problem28(side: i32) -> usize {
    let mut sum = 1;

    // Coordinates of the current point
    let mut x = 0;
    let mut y = 0;
    let mut i = 1; // current number

    // Current state
    let mut circle_nb = 0;
    let mut direction = Direction::Right;


    while circle_nb <= side/2 {
        // Possibly change direction and circle number
        match direction {
            Direction::Down => if y == -circle_nb { direction = Direction::Left; },
            Direction::Left => if x == -circle_nb { direction = Direction::Up; },
            Direction::Up => if y == circle_nb { direction = Direction::Right; },
            Direction::Right => if x == circle_nb+1 { 
                direction = Direction::Down; 
                circle_nb += 1;
            },
        }

        // Move the point
        match direction {
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Up => y += 1,
            Direction::Right => x += 1,
        }
        
        // Add to sum if on a diagonal
        i += 1;
        if x == y || x == -y {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem28() {
        assert_eq!(problem28(5), 101);
        assert_eq!(problem28(1_001), 669_171_001);
    }
}