use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let k = input.next().unwrap();

    let mut num: Vec<i32> = Vec::with_capacity(n as usize);
    let mut result = 0;
    let mut count = 0;

    for _ in 0..n {
        num.push(input.next().unwrap());
    }

    num.sort_by(|a, b| b.cmp(a));

    // m번만큼 k개중복가능
    // 첫 번째수와 두 번째 수만 사용

    while count < m {
        for _ in 0..k {
            if count != m {
                result += num[0];
                count += 1;
            }
        }

        if count != m {
            result += num[1];
            count += 1;
        }
    }

    println!("{}", result);

    // 수열을 사용 한 풀이
    // 가장 큰 수 사용 횟수 구하기
    // let mut count = (m / (k + 1)) * k + m % (k + 1);
    // result += count * num[0];
    // result += (m - count) * num[1];
}
