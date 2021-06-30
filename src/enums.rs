enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_character(dir: Movement) {
    match dir {
        Movement::Up => println!("Moving Up"),
        Movement::Down => println!("Moving Down"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right"),
    }
}

pub fn run() {
    let player1 = Movement::Left;
    let player2 = Movement::Right;
    let player3 = Movement::Up;
    let player4 = Movement::Down;

    move_character(player1);
    move_character(player2);
    move_character(player3);
    move_character(player4);
}
