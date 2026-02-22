pub fn main() {
    let largest_val = largest(&['y', 'x', 'a', 'z']);
    println!("{}", largest_val);
}

// a function that looks for largest int or char
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// with structs.
//
// u can declare as many types as u want.
struct Point<T, U> {
    x: T,
    y: U,
}

// using impls with generics
// impl<GENERICS> type<GENERICS> {
//
// }

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn with_structs<T>() {
    let integer: Point<i32, i32> = Point { x: 5, y: 9 };
    let float: Point<f32, f32> = Point { x: 1.5, y: 3.2 };
    let string: Point<char, char> = Point { x: 'c', y: 'b' };
}

// generics for enums.
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
