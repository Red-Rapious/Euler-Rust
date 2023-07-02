/*
Problem 4:
A palindromic number reads the same both ways. 
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 * 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn is_palindrom(n: i32) -> bool {
    let number: Vec<char> = n.to_string().chars().collect();
    let mut pal = true;

    for i in 0..(number.len()/2) {
        if number[i] != number[number.len() - 1 - i] { 
            pal = false;
            break; 
        }
    }
    pal
}

pub fn problem4() -> i32 {
    let mut max = 0;

    // Loops through every single couple of 3-digits numbers
    // A better, more complex, approach might be to loop by through i and j such as
    // the (i*j) sequence is always decreasing, and starts at 999*999
    for i in 100..1000 {
        for j in 100..1000 {
            if i*j > max && is_palindrom(i*j) {
                max = i*j;
            }
        }
    }
    max
}