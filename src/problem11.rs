/* 
Problem 11:
In the 20 × 20 grid below, four numbers along a diagonal line have been marked in red.

The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
What is the greatest product of four adjacent numbers in the same direction 
(up, down, left, right, or diagonally) in the 20 × 20 grid?
*/

use std::fs;

pub fn problem11() -> i32 {
    let g = get_grid();
    let mut maxi = 0;
    
    // Horizontal
    for i in 0..20 {
        for j in 0..20-4 {
            let p = g[i][j] * g[i][j+1] * g[i][j+2] * g[i][j+3];
            if p > maxi { maxi = p; }
        }
    }

    // Vertical
    for i in 0..20-4 {
        for j in 0..20 {
            let p = g[i][j] * g[i+1][j] * g[i+2][j] * g[i+3][j];
            if p > maxi { maxi = p; }
        }
    }

    // Diagonal \
    for i in 0..20-4 {
        for j in 0..20-4 {
            let p = g[i][j] * g[i+1][j+1] * g[i+2][j+2] * g[i+3][j+3];
            if p > maxi { maxi = p; }
        }
    }

    // Diagonal /
    for i in 0..20-4 {
        for j in 4..20 {
            let p = g[i][j] * g[i+1][j-1] * g[i+2][j-2] * g[i+3][j-3];
            if p > maxi { maxi = p; }
        }
    }

    maxi
}

fn get_grid() -> [[i32; 20] ; 20] {
    let file_path = "src/problem11.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    let mut array: [[i32; 20]; 20] = [[0; 20]; 20];

    let mut i = 0;
    for line in contents.split("\n") {
        let mut j = 0;
        for number in line.split(" ") {
            array[i][j] = number.parse().unwrap();
            j += 1;
        }
        i += 1;
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem11() {
        assert_eq!(problem11(), 70_600_674);
    }
}