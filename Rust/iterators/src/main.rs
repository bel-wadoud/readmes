fn main() {
    // 1.
    let v1: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<i32> = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{val}");
    }

    println!("==========");

    for val in v2.iter_mut() {
        *val += 1;
        println!("{val}");
    }

    // .into_iter() transfers the ownership/

    // Map
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("{:?}", squares);

    // Filter
    let even: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", even);

    // .fold()
    // .fold() is how you reduce an iterator into a single value.
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("{sum}");
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
