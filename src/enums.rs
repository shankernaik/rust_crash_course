enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn move_av(m: Movement) {
    match m {
        Movement::UP => println!("Move up"),
        Movement::DOWN => println!("Move down"),
        Movement::LEFT => println!("Move left"),
        Movement::RIGHT => println!("Move right"),
    }
}

pub fn run() {
    let av1 = Movement::LEFT;
    let av2 = Movement::RIGHT;
    let av3 = Movement::UP;
    let av4 = Movement::DOWN;

    move_av(av1);
    move_av(av2);
    move_av(av3);
    move_av(av4);
}
