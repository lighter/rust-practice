use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get array lenth
    println!("Array length: {}", numbers.len());

    // Array are stack allocated
    println!("Array occupise {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}