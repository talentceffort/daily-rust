use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let k = input.next().unwrap().parse::<usize>().unwrap();

    let mut vec_a: Vec<usize> = Vec::with_capacity(n);
    let mut vec_b: Vec<usize> = Vec::with_capacity(n);

    for i in 0..2 {
        for _ in 0..n {
            if i == 0 {
                vec_a.push(input.next().unwrap().parse::<usize>().unwrap())
            } else {
                vec_b.push(input.next().unwrap().parse::<usize>().unwrap());
            }
        }
    }

    vec_a.sort_by(|a, b| a.cmp(b));
    vec_b.sort_by(|a, b| b.cmp(a));

    for i in 0..k {
        if vec_a[i] < vec_b[i] {
            // rust도 파이썬처럼 배열 스위칭 가능. but 튜플 사용
            (vec_a[i], vec_b[i]) = (vec_b[i], vec_a[i]);
        } else {
            break;
        }
    }

    // sum도 할 수 있음.
    let sum: usize = vec_a.iter().sum();

    println!("{}", sum);
}
