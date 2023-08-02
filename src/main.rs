use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let squares = [
        [[8,1,6], [3,5,7][4,9,2]],
        [[8,1,6], [3,5,7][4,9,2]],
    ];
    1
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = formingMagicSquare(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
