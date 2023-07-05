/*
Problem 17:
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 
3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters 
and 115 (one hundred and fifteen) contains 20 letters. 
The use of "and" when writing out numbers is in compliance with British usage.
*/

fn digit_to_letters(d: i32) -> i32 {
    assert!(0 <= d && d < 10);
    match d {
        0 => 0, // silent
        1 => 3, // one
        2 => 3, // two
        3 => 5, // three
        4 => 4, // four
        5 => 4, // five
        6 => 3, // six
        7 => 5, // seven
        8 => 5, // eight
        9 => 4, // nine
        _ => -1
    }
}

fn tens_to_letters(b: i32, c: i32) -> i32 {
    assert!(0 <= b && b < 10);
    assert!(0 <= c && c < 10);

    match b {
        0 => digit_to_letters(c),
        2 => digit_to_letters(c) + 6, // twenty
        3 => digit_to_letters(c) + 6, // thirty
        4 => digit_to_letters(c) + 5, // forty
        5 => digit_to_letters(c) + 5, // fifty
        6 => digit_to_letters(c) + 5, // sixty
        7 => digit_to_letters(c) + 7, // seventy
        8 => digit_to_letters(c) + 6, // eighty
        9 => digit_to_letters(c) + 6, // ninety
        1 => match c {
            0 => 3, // ten
            1 => 6, // eleven
            2 => 6, // twelve
            3 => 8, // thirteen
            4 => 8, // fourteen
            5 => 7, // fifteen
            6 => 7, // sixteen
            7 => 9, // seventeen
            8 => 8, // eighteen
            9 => 8, // nineteen
            _ => -1
        }
        _ => -1
    }
}

fn number_of_letters(n: i32) -> i32 {
    if n == 1_000 {
        return 3 + 8 // one thousand
    } else {
        let a = n / 100;
        let b = (n/10) % 10;
        let c = n % 10;

        let hundreds = digit_to_letters(a) + if a != 0 {7 } else { 0 }; // hundred
        let tens = tens_to_letters(b, c);

        return hundreds + tens + if hundreds != 0 && tens != 0 { 3 /* and */ } else { 0 }
    }
}

pub fn problem17() -> i32 {
    let mut s = 0;
    for i in 1..1_001 {
        s += number_of_letters(i);
    }
    s
}