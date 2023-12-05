use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    // 맵의 크기
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    // 서있는 좌표
    let mut x = input.next().unwrap();
    let mut y = input.next().unwrap();

    // 방향
    let mut d = input.next().unwrap();

    println!("d {}", d);
    println!("x {}", x);
    println!("y {}", y);
    println!("n {}", n);
    println!("m: {}", m);

    // 북 서 남 동
    // 왼쪽 방향
    let dx = [-1, 0, 1, 0];
    let dy = [0, -1, 0, 1];

    let mut map: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    let mut visited: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];

    println!("visited ->> {:?}", visited);

    for _ in 0..n {
        let mut row = Vec::with_capacity(m as usize);
        for _ in 0..m {
            row.push(input.next().unwrap());
        }
        map.push(row);
    }

    println!("{:?}", map);

    visited[x as usize][y as usize] = 1;
    let mut count = 1;
    let mut turn_time = 0;
    loop {
        // 캐릭터의 첫 왼쪽 방향을 구한다.
        d = turn_left(&d);

        let mut mx = 0;
        let mut my = 0;

        mx = x + dx[d as usize];
        my = y + dy[d as usize];

        if visited[mx as usize][my as usize] == 0 && map[mx as usize][my as usize] == 0 {
            count += 1;
            x = mx;
            y = my;
            turn_time = 0;
            continue;
        } else {
            turn_time += 1;
        }

        if turn_time == 4 {
            // 뒷방향 좌표 구하기. 현재 좌표에서 그대로 빼주기만 하면 뒷방향임
            mx = x - dx[d as usize];
            my = y - dy[d as usize];

            // 뒤로 갈 수 있다면 이동
            if visited[mx as usize][my as usize] == 0 {
                x = mx;
                y = my;
            }
            break;
        }
    }
    println!("{}", count);
}

fn turn_left(&d: &i32) -> i32 {
    let mut result = d;
    result += 1;
    if result > 3 {
        return 1;
    }
    result
}
