pub fn main() {
    let closure = || "Hello world";

    println!("{}", closure());

    let add = |a: i32, b: i32| a + b;

    println!("some {}", add(4, 3));

    // there are 3 ways to capture vars by closures
    // 1. borrowing a variable immutably
    // 2. borrowing a variable mutably
    // 3. taking the ownership of the variable

    // 1)
    let x: i32 = 4;

    let print_x = || println!("{x}");

    print_x();

    // 2)
    let mut y: i32 = 3;

    let mut print_y = || {
        y += 1;
        println!("{y}");
    };

    print_y();

    // 3)
    let z: i32 = 12;

    let print_z = move || {
        println!("{z}");
        drop(z);
    };

    print_z();
}

// closures as function params
fn tomato<T>(t: T)
where
    T: Fn(i32) -> i32,
{
    println!("{}", t);
    4
}
