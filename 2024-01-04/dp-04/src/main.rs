use std::{
    cmp::min,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();

    println!("{n}");
    println!("{m}");

    // m을 만들어야 함.

    let mut coins = Vec::new();
    for _ in 0..n {
        coins.push(input.next().unwrap().parse::<usize>().unwrap());
    }

    let mut dp = vec![10_001; m + 1];
    dp[0] = 0;

    // 점화식은 dp[i] = min(dp[i], dp[i - coins[j]] + 1)
    // 이 점화식은 j번째 화폐를 사용하여 금액 i를 만드는 경우를 생각한다
    // dp[i - coins[j]]는 j번째 화폐를 사용하기 전에 필요했던 금액을 만드는 데 필요한 최소한의 화폐 개수
    // 여기에 1을 더하면 j번째 화폐를 사용하여 금액 i를 만드는 데 필요한 화폐 개수를 구할 수 있음.

    for i in 0..n {
        for j in coins[i]..=m {
            dp[j] = min(dp[j], dp[j - coins[i]] + 1);
        }
    }

    if dp[m] == 10_001 {
        println!("-1");
    } else {
        println!("{}", dp[m]);
    }
}
