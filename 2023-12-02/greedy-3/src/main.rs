use std::cmp::max;
use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let mut result = 0;

    for _ in 0..n {
        let mut v: Vec<i32> = Vec::with_capacity((m as i32).try_into().unwrap());
        for _ in 0..m {
            v.push(input.next().unwrap());
        }
        let min_value = v.iter().min().unwrap();
        result = max(result, *min_value);
    }

    println!("{}", result);
}
