use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut items: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        let t = input.next().unwrap().parse::<usize>().unwrap();
        items.push(t);
    }

    let m: usize = input.next().unwrap().parse::<usize>().unwrap();

    let mut find: Vec<usize> = Vec::with_capacity(m);

    for _ in 0..m {
        let t = input.next().unwrap().parse::<usize>().unwrap();
        find.push(t);
    }

    items.sort_by(|a, b| a.cmp(b));

    for i in find {
        match items.binary_search(&i) {
            Ok(_) => {
                print!("yes ")
            }
            Err(_) => {
                print!("no ")
            }
        }
    }
    println!()
}
