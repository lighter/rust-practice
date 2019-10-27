use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get vector lenth
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupise {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2 ];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // Loog & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}