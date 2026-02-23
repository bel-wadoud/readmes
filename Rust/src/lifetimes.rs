use std::{fmt::Display, fs::write};

pub fn main() {
    // the &'static lives the entire duration of the program.
    let s: &'static str = "this is a static string that lives the whole program duration";
}

// this will run into an issue
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
*/

// lifetime annotation in structs.
struct Person<'a> {
    part: &'a str,
}

// lifetime annotation in method defenetions.
impl<'a> Person<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let word = String::from("Hollaaa!");
    let person = Person { part: &word };
    x
}

fn longest_with_an_announce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    "hello"
}
// three rules of lifetimes.
// 1. each param which is a reference get's its own lifetime parameter.
// 2. if there's exactly one input lifetime parameter, that input is assigned to all output's
//    lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
//    because this is a method, the lifetime of self is assigned to all output lifetime parameters
