use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<i32>().unwrap();

    let mut commands: Vec<&str> = Vec::new();

    while let Some(line) = input.next() {
        // let line = line;
        // let items = line.split(" ").map(|n| n).collect();
        commands.push(line);
    }

    // println!("n: {:?}", n);
    // println!("res: {:?}", commands);

    let mut map: Vec<Vec<(i32, i32)>> = Vec::with_capacity((n * n) as usize);

    for i in 0..n {
        let mut row: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
        for j in 0..n {
            row.push((i as i32 + 1, j as i32 + 1));
        }
        map.push(row);
    }

    let mut x = 0;
    let mut y = 0;

    let dx = vec![0, 0, -1, 1];
    let dy = vec![-1, 1, 0, 0];
    let move_types = vec!["L", "R", "U", "D"];

    for command in commands {
        let mut nx = x;
        let mut ny = y;
        for i in 0..move_types.len() {
            if command == move_types[i] {
                nx = x + dx[i];
                ny = y + dy[i];
            }
        }
        if nx < 0 || ny < 0 || nx > n - 1 || ny > n - 1 {
            continue;
        }
        x = nx;
        y = ny;
    }

    // for command in commands {
    //     let mut mx = x;
    //     let mut my: i32 = y;

    //     if command.to_string() == "R" {
    //         my += 1
    //     }

    //     if command.to_string() == "L" {
    //         my -= 1
    //     }

    //     if command.to_string() == "U" {
    //         mx -= 1;
    //     }

    //     if command.to_string() == "D" {
    //         mx += 1;
    //     }

    //     if mx < 0 || my < 0 || mx > n - 1 || my > n - 1 {
    //         continue;
    //     }

    //     x = mx;
    //     y = my;
    // }

    println!(
        "{:?} {:?}",
        map[x as usize][y as usize].0, map[x as usize][y as usize].1
    );
}
