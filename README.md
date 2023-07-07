# Euler-Rust
My [Project Euler](https://projecteuler.net/) track in Rust. Every solution is commented, and some problems feature multiple solutions.

## Compiling and Running
### Default `cargo run`
This project uses `Cargo`. With `rust` installed on your machine, you can execute my code with:
```console
$ cargo run
```

The main program then asks for a Problem number to run, which you can enter in the command line:
```console
$ cargo run
    Compiling ...
     Finished ...
      Running ...
Problem number:
1
```

### Environment Argument
Alternatively, you can directly pass the problem number as an environment argument:
```console
$ cargo run 1
    Compiling ...
     Finished ...
      Running ...
Problem 1 solution: ...
```

### Unit Tests
Finally, I have configured unit tests for each problem. You can run them with:
```console
$ cargo test
```
or to run a specific test:
```console
$ cargo test test_problem1
```

Some problems have multiple solutions, and the unit tests are configured to run all of them.
```console
$ cargo test test_problem1_v2
```

Some solutions, such as the one for [Problem 23](src/problem23.rs), take a few more seconds to run than the others. Corresponding tests are ignored by default, but you can run them with:
```console
$ cargo test -- --ignored
```
or to run all tests, ignored or not:
```console
$ cargo test -- --include-ignored
```

## Problems solved
- [Problem 1:](src/problem1.rs) Multiples of 3 or 5
- [Problem 2:](src/problem2.rs) Even Fibonacci Numbers
- [Problem 3:](src/problem3.rs) Largest Prime Factor
- [Problem 4:](src/problem4.rs) Largest Palindrome Product
- [Problem 5:](src/problem5.rs) Smallest Multiple
- [Problem 6:](src/problem6.rs) Sum Square Difference 
- [Problem 7:](src/problem7.rs) 10001st Prime 
- [Problem 8:](src/problem8.rs) Largest Product in a Series
- [Problem 9:](src/problem9.rs) Special Pythagorean Triplet
- [Problem 10:](src/problem10.rs) Summation of Primes
- [Problem 11:](src/problem11.rs) Largest Product in a Grid
- [Problem 12:](src/problem12.rs) Highly Divisible Triangular Number
- [Problem 13:](src/problem13.rs) Large Sum
- [Problem 14:](src/problem14.rs) Longest Collatz Sequence
- [Problem 15:](src/problem15.rs) Lattice Paths
- [Problem 16:](src/problem15.rs) Power Digit Sum
- [Problem 17:](src/problem17.rs) Number Letter Counts
- [Problem 18:](src/problem18.rs) Maximum Path Sum I
- [Problem 19:](src/problem19.rs) Counting Sundays
- [Problem 20:](src/problem20.rs) Factorial Digit Sum
- [Problem 21:](src/problem21.rs) Amicable Numbers
- [Problem 22:](src/problem22.rs) Names Scores
- [Problem 23:](src/problem23.rs) Non-Abundant Sums
- [Problem 24:](src/problem24.rs) Lexicographic Permutations
- [Problem 25:](src/problem25.rs) 1000-digit Fibonacci Number
- [Problem 27:](src/problem27.rs) Quadratic Primes
- [Problem 28:](src/problem28.rs) Number Spiral Diagonals
- [Problem 29:](src/problem29.rs) Distinct Powers

## License
This work is licensed under the [CC-BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license. Original content is the property of the [Project Euler website](https://projecteuler.net/copyright) and also licensed under **CC-BY-NC-SA 4.0**.