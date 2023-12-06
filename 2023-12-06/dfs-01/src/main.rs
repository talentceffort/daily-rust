use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<i32>().unwrap();
    let m = input.next().unwrap().parse::<i32>().unwrap();

    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let mut row: Vec<i32> = Vec::with_capacity(m as usize);
        for word in input.next().unwrap().chars() {
            row.push(word as i32 - 0x30)
        }
        map.push(row)
    }

    // println!("map: {:?}", map);

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            if dfs(i, j, n, m, &mut map) {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

fn dfs(x: i32, y: i32, n: i32, m: i32, map: &mut Vec<Vec<i32>>) -> bool {
    if x <= -1 || x >= n || y <= -1 || y >= m {
        return false;
    }
    if map[x as usize][y as usize] == 0 {
        map[x as usize][y as usize] = 1;
        dfs(x - 1, y, n, m, map);
        dfs(x, y - 1, n, m, map);
        dfs(x + 1, y, n, m, map);
        dfs(x, y + 1, n, m, map);
        return true;
    }
    false
}
