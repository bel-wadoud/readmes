pub fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    let mut s = String::from("best string");

    println!("{}", s);

    // push strings UTF-8
    s.push_str(", from wadoud");
    println!("{}", s);

    // push a single char
    s.push('A');
    println!("{}", s);

    // concatination.
    let s1 = String::from("foo ");
    let s2 = String::from("bar");

    // we need to use a string than a reference.
    // cuz
    // fn add(self, s: &str) -> String {}
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    // concatination using format! macro
    let with_format = format!("{}-{}", s1, s2);
    println!("{}", with_format);

    // Indexing into strings isn't possible while slicing is.

    // iterating over strings.
    for c in "Hello".chars() {
        println!("{}", c);
    }
}
