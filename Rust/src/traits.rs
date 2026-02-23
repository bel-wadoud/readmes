use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsLetter {
    headline: String,
    author: String,
    content: String,
    locatoin: String,
}

impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        format!("{}-{}-{}", self.author, self.headline, self.locatoin)
    }
}

// traits as params
// T is a pattern (we define it in <>), after that in params we say gimme anything that implements
// that pattern.
// we can use multiple traits by using + sign, (Summary + Display)
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

pub fn random_func<T: Display + Clone, U: Display + Summary>(inp: &T, outp: &U) {}
// this can be written in a much cleaner way using "where" clause
pub fn random_func2<T, U>(inp: &T, outp: &U) -> i32
where
    T: Display + Clone,
    U: Display + Summary,
{
    0
}

// returning a trait as an output in a function.
// Note: in a function where a Trait is returned we can't implement logic.
fn return_trait() -> impl Summary {
    NewsLetter {
        headline: "test head".to_string(),
        author: "test author".to_string(),
        content: "test content".to_string(),
        locatoin: "test location".to_string(),
    }
}

pub fn main() {
    let news: NewsLetter = NewsLetter {
        headline: "tomato".to_string(),
        author: "wadoud".to_string(),
        content: "nothing important".to_string(),
        locatoin: "Algeria".to_string(),
    };

    println!("{}", news.summarize());
}

// using trait bounds to conditionally implement methods.

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// PartialOrd is used to compare data from different types
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}
