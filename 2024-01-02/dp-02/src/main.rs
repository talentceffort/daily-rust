use std::{
    cmp::max,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    println!("{n}");

    let k: Vec<usize> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();

    let mut memo = vec![0; 100];

    memo[0] = k[0];
    memo[1] = max(k[0], k[1]);

    for i in 2..n {
        memo[i] = max(memo[i - 1], memo[i - 2] + k[i])
    }

    println!("{:?}", memo[n - 1]);
}
