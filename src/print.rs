pub fn run() {
    println!("hello rs file");

    // Basic format
    println!("Number: {}", 1);
    println!("{} is from {}", "A", "B");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "A", "B", "C");

    // Name arguments
    println!("{name} likes to play {activity}", name = "Joh", activity="Baseball");

    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // placehoder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}