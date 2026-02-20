use std::collections::HashMap;

// HashMap is literally a "JSON object" in concept

pub fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 18);
    scores.insert(String::from("Orange"), 13);

    println!("{:?}", scores);

    // getting elements.
    let blue = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("{}", blue);

    // iterration.
    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    // only inserting a value if the key has no value.
    let mut map5 = HashMap::new();

    map5.insert(String::from("Blue"), 1);
    map5.entry(String::from("Blue")).or_insert(3);

    println!("{:?}", map5);
}
