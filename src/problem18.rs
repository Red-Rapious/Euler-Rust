/*
Problem 18:
By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
3
7 4
2 4 6
8 5 9 3
That is, 3 + 7 + 4 + 9 = 23.
Find the maximum total from top to bottom of the triangle below:

NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! */

use std::cmp::max;
use std::fs;

fn shortest_path_length(triangle: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    if y == triangle.len() - 1 {
        return triangle[y][x]
    } else {
        return max(
            shortest_path_length(&triangle, x, y+1),
            shortest_path_length(&triangle, x+1, y+1)
        ) + triangle[y][x]
    }
}

pub fn problem18() -> usize {
    let triangle: Vec<Vec<usize>> = get_triangle();
    shortest_path_length(&triangle, 0, 0)
}

fn get_triangle() -> Vec<Vec<usize>> {
    let file_path = "src/problem18.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file.");

    let mut triangle = vec![];

    let mut i = 0;
    for line in contents.split("\n") {
        triangle.push(vec![]);
        for number in line.split(" ") {
            triangle[i].push(number.parse().unwrap());
        }
        i += 1;
    }
    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem18() {
        assert_eq!(problem18(), 1_074);
    }
}