use std::fmt::Write;
use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse::<usize>().unwrap();

    let mut vec: Vec<(String, usize)> = Vec::with_capacity(n);

    for _ in 0..n {
        // Read the line and split it into words
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut words = input.split_whitespace();

        // Extract data and create a tuple
        if let (Some(key), Some(value)) = (words.next(), words.next()) {
            let tuple = (key.to_string(), value.parse::<usize>().unwrap());
            println!("{:?}", tuple); // Replace this with your desired processing
            vec.push(tuple);
        }
    }

    vec.sort_by(|a, b| a.1.cmp(&b.1)); // increasing order

    println!("vec: {:?}", vec);

    vec.sort_by(|a, b| b.1.cmp(&a.1)); // decreasing order

    println!("vec: {:?}", vec);

    // 다양한 출력 방식

    // 출력 1
    for (str_value, _) in &vec {
        print!("{} ", str_value);
    }
    println!();

    // 출력 2
    let mut output = String::new();
    for (str_value, _) in &vec {
        write!(output, "{} ", str_value).unwrap();
    }
    println!("{}", output.trim())
}
