#![allow(dead_code)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let s = Directions::Up;

    println!("direction is: {}", call(s));
}

fn call(Directions: Directions) -> String {
    match Directions {
        Directions::Up => "Uppp".to_string(),
        Directions::Down => "Down".to_string(),
        Directions::Right => "Right".to_string(),
        Directions::Left => "Left".to_string(),
    }
}

fn plus_on(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// another way to manage Option in match statement.
fn if_let(x: Option<i32>) -> Option<i32> {
    let config_x: Option<i32> = Some(100);

    if let Some(x_val) = config_x {
        return Some(0);
    } else {
        return Some(1);
    }
}
