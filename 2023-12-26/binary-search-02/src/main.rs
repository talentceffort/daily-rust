use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();

    println!("input: {n}");
    println!("input: {m}");

    let mut array: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        array.push(input.next().unwrap().parse::<usize>().unwrap());
    }

    array.sort();

    let mut start = 0;
    let mut end = *array.iter().max().unwrap();
    let mut result = 0;
    while start <= end {
        let mid = (start + end) / 2;
        let total: usize = array.iter().filter(|&&x| x > mid).map(|&x| x - mid).sum();

        // for &x in &array {
        //     if x > mid {
        //         total += x - mid;
        //     }
        // }

        // 떡의 양이 부족. 더 많이 잘라야함. 끝점 낮추기
        if total < m {
            end = mid - 1;
        } else {
            // 떡의 양이 충분. 덜 잘라야함. 시작점 높이기
            result = mid;
            start = mid + 1;
        }
    }

    println!("{result}")
}
