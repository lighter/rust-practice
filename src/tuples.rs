pub fn run() {
    let person: (&str, &str, i8) = ("a", "b", 18);

    println!("{}, {}, {}", person.0, person.1, person.2);   
}