/*
Problem 22:
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, 
begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, 
multiply this value by its alphabetical position in the list to obtain a name score.
For example, when the list is sorted into alphabetical order, 
COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. 
So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
*/

use std::fs;

fn score(position: usize, name: &str) -> usize {
    let a = name.chars().map(|c| c as usize - 'A' as usize + 1);
    let mut s = 0;
    for e in a {
        s += e;
    }
    s * position
}

pub fn problem22() -> usize {
    let contents = fs::read_to_string("src/problem22.txt")
        .expect("Unable to read file.").replace("\"", "");

    let mut names = vec![];
    for name in contents.split(",") {
        names.push(name);
    }

    names.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut s = 0;
    for i in 0..names.len() {
        s += score(i+1, names[i]);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem22() {
        assert_eq!(problem22(), 871_198_282);
    }
}