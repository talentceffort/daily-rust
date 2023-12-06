fn modify_data(map: &mut Vec<Vec<i32>>) {
    // Access and modify the content of the vector using the mutable reference
    for row in map.iter_mut() {
        for element in row.iter_mut() {
            *element += 1; // Increment each element by 1
        }
    }
}

fn consume_data(mut map: Vec<Vec<i32>>) {
    // Take ownership of the vector and modify or consume it
    for row in map.iter_mut() {
        for element in row.iter_mut() {
            *element += 1; // Increment each element by 1
        }
    }
    // The vector `map` is owned by this function and will be dropped when it goes out of scope
}

// call modify_data
fn main() {
    // Create a vector of vectors
    let mut data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Call the function with a mutable reference to the vector
    modify_data(&mut data);

    // Now `data` has been modified in place
    println!("{:?}", data);
}

// call consume_data
fn main() {
    // Create a vector of vectors
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Call the function, transferring ownership of the vector to the function
    consume_data(data);

    // Attempting to use `data` here would result in a compilation error because ownership was transferred
}
