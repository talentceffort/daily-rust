use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut n = input.next().unwrap();
    let k = input.next().unwrap();

    // 1. n에서 1을 뺀다
    // 2. n을 k로 나눈다
    // n이 1일 될 때 까지 1번 혹은 2번의 전체 실행 횟수 리턴.

    let mut count = 0;

    // while n >= k {
    //     while n % k != 0 {
    //         n -= 1;
    //         count += 1;
    //     }
    //     n /= k;
    //     count += 1;
    // }

    // while n > 1 {
    //     n -= 1;
    //     count += 1;
    // }

    // println!("{}", count);

    loop {
        // n이 k의 배수가 되도록 한 번에 빼기
        let target = (n / k) * k; // n = 10, k = 3 일 때 target은 9가 됨
        count += n - target;
        n = target;

        if n < k {
            break;
        }
        n /= k;
        count += 1;
    }

    count += n - 1;
    println!("{}", count)
}
