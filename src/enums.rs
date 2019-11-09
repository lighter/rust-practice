enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right
}

fu move(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() {
    lat avatar1 = Movement::Left;
    lat avatar2 = Movement::Up;
    lat avatar3 = Movement::Right;
    lat avatar4 = Movement::Down;

    move(avatar1);
    move(avatar2);
    move(avatar3);
    move(avatar4);
}