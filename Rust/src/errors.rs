use std::fs::File;
use std::io::Error;

pub fn main() {
    // declare an error in the program
    // panic!("let the world burn!");

    // errors with files.
    let greeting_from_file: Result<File, Error> = File::open("hello.txt");
    let _greeting_file: File = match greeting_from_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };

    // alternative approach with if-else.
    // can't demontrate duo to lac of docs.

    // unwrap is a shortcut method that returns the value if OK or panics of Err
    let _gr_file: File = File::open("hello.txt").unwrap();
    // expect() let's u edit the error msg
    let _gr_file_expect: File = File::open("hello.txt").expect("the file doesn't exist baka.");

    let _ = read_file();
}

fn read_file() -> Result<File, std::io::Error> {
    let file = File::open("data.txt")?;
    Ok(file)
}
