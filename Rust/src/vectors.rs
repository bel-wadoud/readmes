pub fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("v1: {:?}", v1);

    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);

    let mut v3: Vec<i32> = vec![1, 2, 3];

    v3.push(4);
    v3.push(5);
    v3.push(6);

    println!("v3: {:?}", v3);

    // delets last element.
    v3.pop();

    println!("v3: {:?}", v3);

    // reading elements
    let fourth: &i32 = &v3[3];
    println!("4th element: {}", fourth);

    let fifth: Option<&i32> = v3.get(4);
    match fifth {
        None => println!("5th element doesn't exist"),
        Some(&fifth) => println!("5th element is: {}", &fifth),
    }

    // iterating over a vector.
    println!("\n iterating. \n");
    for i in v3 {
        println!("{}", i);
    }

    // modifing.
    let mut v4 = vec![10, 11, 12, 13, 14];
    for i in &mut v4 {
        *i += 1;
    }

    println!("v4: {:?}", v4);

    // store multiple types in a vector
    //
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Types {
        Letter(char),
        Text(String),
        Num(i32),
    }

    let row = vec![
        Types::Text(String::from("username: wadoud")),
        Types::Num(05428425),
        Types::Letter('A'),
    ];

    println!("MultiType: {:?}", row);
} // all vectors will go out of scope (empty)

