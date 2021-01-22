/*
 * Puzzle from advent of code 2020, day 1
 *
 * Puzzle instructions
 *
 * Inside the file called `input` there are two entries that sum to 2020
 * The goal of this puzzle is to read the file, find these entries
 * and return both the two entries and the product of them.
 *
 * Given the following input
 *
 * 1456
 * 979
 * 366
 * 1721
 * 299
 * 675
 *
 * The two numbers that sums to 2020 are `1721` and `299` and the product of them is `514579`
 *
 * Try to:
 *
 * 1. Define the output you expect from the function before you start your hacking, it be for example
 *    a) a string -> 'The numbers are 1721 and 299, its product is 514579',
 *    b) a simple array -> [1721, 299, 514579]
 *    c) a tuple -> ("Product: 514579", 1721, 299) or anything else that comes to your mind
 *    It will add an extra challenge, preventing you to fallback to an easier solution and allows
 *    you to `TDD` your code.
 *
 * 2. Modify the test example to match the output you decided on the previous item. At first it's
 *    okay if you hard code the values, after all, you don't know the values yet. E.g.: `The numbers are 0, 0 and 0`
 *
 * 3. Start hacking. Adapt the dummy function below to return the wanted output and start to write
 *    the solution. Run `cargo test` periodically to see the warnings and the assertion errors. If
 *    you installed cargo-watch https://github.com/passcod/cargo-watch let it running in a separated terminal
 *
 * PS: You will find a function called read_file down below. To check it's content go to the
 * utilities folder present at the root of this project. There's nothing special about that though, it was created
 * just to save some time searching the N possibilities on how to import a file.
 *
 * */

use utilities::read_file;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_find_two_that_sums_to_2020() {
        assert_eq!(find_two(), None);
    }
}

fn find_two() -> Option<String> {
    println!("{:?}", read_file("src/input"));
    None
}

fn main() {
    println!("Two numbers that sums to 2020 are {:?}", find_two())
}
