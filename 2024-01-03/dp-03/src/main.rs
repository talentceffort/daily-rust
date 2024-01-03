use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    // 가로의 길이가 N, 세로의 길이가 2
    // 1 X 2, 2 X 1, 2 X 2 로 채우려 할 때 모든 경우의 수

    let mut dp = vec![0; 1000];

    dp[0] = 1;
    dp[1] = 3;

    for i in 2..n {
        // dp[i -1]에는 이미 1x2 타일이 가로로 2개 연속으로 놓이는 경우도 포함
        // 그러므로 dp[i - 2]에서 가로로 2개 놓이는 경우의 수는 고려하지 않음.
        dp[i] = dp[i - 1] + (dp[i - 2] * 2);
    }

    println!("dp3 : {:?}", dp[n - 1]);
}
