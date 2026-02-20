mod vectors;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    phone: i64,
    is_active: bool,
    age: u8,
}

// tuple like structs
struct _Point(u8, u8, u8);

// unit-like struct
struct _Color;

struct Rectangle {
    width: u32,
    height: u32,
}
/usr/bin/bash: line 1: :q: command not found
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods with the same name of fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    // getter for height
    fn height(&self) -> u32 {
        self.height
    }

    // check if a Rectangle can hold another.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function to define a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut s = String::from("Rust!");

    let user1 = User {
        // borrowing works here too (in creating another instance of this instance.)
        name: String::from("Wadoud"),
        email: String::from("wadoud@mail.com"),
        phone: 0542894258,
        is_active: true,
        age: 19,
    };

    let square = Rectangle::square(9);

    println!("user1 instance: {:#?}", user1);

    println!("name: {}", user1.name);

    // change_borowed_value(&mut s);
    // slices();

    // let string1 = "I'm love chada!";
    // println!("the output is: {}", execrice_slices(&string1));

    // s.clear(); // strings value becomes ""
}

fn _change_borowed_value(s: &mut String) {
    s.push_str(" The Love!");
}

fn _call_string(s: String) {
    println!("called string is: {}", s);
}

fn _give_ownership() -> String {
    let some_string = String::from("give_ownership string!");
    some_string
}

fn _control_flow() {
    let boolean = true;
    let list = [1, 2, 3, 4, 5];
    let mut numbers = [1, 2, 3];

    if boolean {
        println!("the param is true");
    } else {
        println!("the param is false!");
    }

    for item in list.iter() {
        println!("the number is {}", item);
    }

    for item in numbers.iter_mut() {
        println!("mut number is {}", item);
    }
}

fn _heap() {
    let s = String::from("Hello");
    println!("Hello, world!");

    let s2 = s.clone();

    println!("string is: {}, s2 {}", s, s2);
}

fn _slices() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];

    println!("{:?}", slice);

    let s = String::from("Hello World!");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {}, world: {}", hello, world);

    // shortcut for initial index
    let _three = &s[..3];

    // shortcut for final index
    let length = s.len();
    let _last = &s[3..length];
}

fn execrice_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    // println!("bytes: {}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn enums() {
    enum IPAddrKind {
        V4,
        V6,
    }

    struct IPAddr {
        kind: IPAddrKind,
        address: String,
    }

    let four = IPAddrKind::V4;

    let localhost = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _username: Option<String> = String::from("My name!");

    // matches
    enum Directions {
        Up,
        Down,
        Right,
        Left,
    }

    /*
     * match MATCH {
     *  Value => Value's Value,
     *  _ => default value
     * }
     * */

    match Directions {
        Up => "Go Up",
        Down => "Go Down",
        Right => "Go right",
        Left => "Go Left",
        _ => "Go no where",
    }
}
