pub fn run() {
    let name = "blabla";
    let mut age = 18;
    age = 19;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ted", 18);
    println!("My name is {} and I am {}", my_name, my_age);
}