use dotenv::dotenv;
use std::env::{self, VarError};
use std::fs::File;

// read used for .read_to_string
//
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    // let content = read_file_to_string("foo.txt");
    // println!("{:?}", content);
    // match content {
    //    Ok(file_content) => println!("content: {}", file_content),
    //    Err(e) => println!("Eroror reading file {}", e),
    // }

    // println!("line by line");
    //read_line_by_line("data");
    //
    env_vars();
}

fn _read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    // let contents: String = fs::read_to_string(file_path).expect(msg);
    Ok(contents)
}

fn _read_line_by_line(_filename: &str) -> io::Result<()> {
    let file = File::open("foo.txt")?;

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line?;
        println!("{line}");
    }

    Ok(())
}

fn env_vars() {
    let key: &str = "AAA";
    unsafe {
        env::set_var(key, "test");
    }

    // read the env vars from a file.
    dotenv().ok(); // load the .env file 
    // to just get the value of the var not an inner result object, just use .unwrap() or better
    // use .except("error msg") so it clearifies when the program craches.
    let api_key: Result<String, VarError> = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("api key value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
