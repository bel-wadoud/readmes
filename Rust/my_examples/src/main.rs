struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let v = vec![1, 2, 3, 4];

    println!("{:?}", v.iter().take(2).collect::<Vec<_>>());

    let mut x = Some(5);

    let y = x.take();

    println!("{:?} \n {:?}", x, y);

    let P = Point { x: 0, y: 2 };

    let Point { x: a, y: b } = P;

    println!("a: {}, b: {}", a, b);

    let mut num: i32 = 5;

    let r1: *const i32 = &num as *const i32;
    let r2: *mut i32 = &mut num as *mut i32;

    println!("{:?} {:?}", r1, r2);

    // unallowed *r1, *r2 here
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    unsafe fn dang() {
        println!("Hello unsafe");
    };

    unsafe {
        dang();
    }
}
