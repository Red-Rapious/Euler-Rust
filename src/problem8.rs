/*
Problem 8:
The four adjacent digits in the 1000-digit number that have the greatest product are 9*9*8*9=5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. 
What is the value of this product?
*/

use std::fs;

pub fn problem8() -> i128 {
    let nb_consecutive = 13;
    let number: Vec<i32> = get_number();

    let mut max = 0;
    for i in 1..(number.len() - nb_consecutive + 1) {
        let mut p = 1;
        for i in i..i+nb_consecutive {
            p *= number[i] as i128
        }
        if p > max { max = p; }
    }
    max
}

/*
Disclaimer: this optimised idea is not working. Multiple following zeros are not supported. 
The idea remains, but is slightly too complex to be needed for such values of nb_consecutive.
*/
pub fn problem8_v2() -> i32 {
    let nb_consecutive = 13;
    let number: Vec<i32> = get_number();

    // Note: the original 13-digits of the number do not contain 0, which is convenient (cf. later)
    let mut p = 1;
    for i in 0..nb_consecutive {
        p *= number[i]
    }

    let mut max = p;
    for i in 1..(number.len() - nb_consecutive + 1) {
        // The idea to avoid calculating each time an nb_consecutive-numbers product is to divide and multiply 
        // each number when it enters or leaves the sequence analysed
        let mut prev = number[i-1] as i32;
        let mut next = number[i+nb_consecutive-1] as i32;

        // This method has one downside: mutltiplication by zero "absorbs" previous answers
        // To fix this, it replaces 0 by -p ; therefore, it assures that the new p is negative, 
        // hence less than the maximum product
        if prev == 0 { prev = if p >= 0 { -1 } else { 1 }; }
        if next == 0 { next = if p >= 0 { -1 } else { 1 }; }

        p /= prev;
        p *= next;
        if p > max { max = p; }
    }
    max
}

fn get_number() -> Vec<i32> {
    let file_path = "problem8.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file.");
    let parsed = contents.replace("\n", "");
    parsed.chars().map(|c| c as i32 - '0' as i32).collect()
}