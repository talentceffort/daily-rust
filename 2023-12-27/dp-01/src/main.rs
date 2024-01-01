use std::{
    cmp::min,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let mut memo: Vec<usize> = vec![0; 30001];

    let x: usize = input.next().unwrap().parse::<usize>().unwrap();

    println!("{:?}", x);

    for i in 2..x + 1 {
        memo[i] = memo[i - 1] + 1;

        if i % 2 == 0 {
            memo[i] = min(memo[i], memo[i / 2] + 1)
        }

        if i % 3 == 0 {
            memo[i] = min(memo[i], memo[i / 3] + 1)
        }

        if i % 5 == 0 {
            memo[i] = min(memo[i], memo[i / 5] + 1)
        }
    }

    println!("{}", memo[x]);
}
