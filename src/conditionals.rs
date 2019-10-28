pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Greater then 21");
    } else if age < 21 && check_id {
        println!("Less then 21");
    } else {
        println!("blablabla", )
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}