use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<String>);

    let n = input.next().unwrap();

    // 0x30 is '0' in ASCII table, easy enough to remember!
    // '1'은 char 타입을 가진다. i32 타입으로 변경하려면 my_char as i32 - 0x30 해준다.
    // a = 98, b = 97

    let row = (n.as_bytes()[1]) as i32 - 0x30; // 행
    let column = (n.as_bytes()[0] as u8 - b'a' + 1) as i32; // 열

    // println!("row: {}", row);
    // println!("column: {}", column);

    let mut count = 0;

    let steps: [(i32, i32); 8] = [
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
    ];

    for step in steps {
        let next_row = row + step.0;
        let next_column = column + step.1;

        if next_row >= 1 && next_row <= 8 && next_column >= 1 && next_column <= 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
