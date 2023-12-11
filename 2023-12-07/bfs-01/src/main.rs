use std::{
    i32,
    io::{stdin, Read},
};

use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<i32>().unwrap();
    let m = input.next().unwrap().parse::<i32>().unwrap();

    println!("input: {:?}", input);
    println!("n {}", n);
    println!("m {}", m);

    let mut map = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let mut row = Vec::with_capacity(m as usize);
        for c in input.next().unwrap().chars() {
            row.push(c as i32 - 0x30);
        }
        map.push(row)
    }

    println!("{:?}", bfs(0, 0, n, m, &mut map));

    println!("map: {:?}", map);
}

fn bfs(x: i32, y: i32, n: i32, m: i32, map: &mut Vec<Vec<i32>>) -> i32 {
    // 상하좌우
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, -1, 1];

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    queue.push_back((x, y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        // 현재 위치에서 4방향 확인
        for i in 0..4 {
            let nx = x + dx[i];
            let ny = y + dy[i];

            if nx < 0 || ny < 0 || nx >= n || ny >= m {
                continue;
            }

            if map[nx as usize][ny as usize] == 0 {
                continue;
            }

            if map[nx as usize][ny as usize] == 1 {
                map[nx as usize][ny as usize] = map[x as usize][y as usize] + 1;
                queue.push_back((nx, ny));
            }
        }
    }
    return map[(n - 1) as usize][(m - 1) as usize];
}
