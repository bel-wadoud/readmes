use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.len() < 2 {
        println!("please enter a string");
        return;
    }
    let mut input: String = args[1].clone();

    println!("`{input}`");
}
