use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<i32>().unwrap();

    // 00시 00분 00초부터 N시 59분 59초까지
    // 총 경우의 수 86400.
    println!("{}", n);

    let mut count = 0;

    for h in 0..n + 1 {
        for m in 0..60 {
            for s in 0..60 {
                let word = h.to_string() + &m.to_string() + &s.to_string();

                if word.contains('3') {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
