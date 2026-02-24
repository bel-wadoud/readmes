pub fn main() {
    println!("Tests");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: usize = add(2, 3);
        assert_eq!(result, 8);
        // other amazing tools
        // #[ignore] // to ignore tests
        // assert!(
        //    result.contains("Carol"),
        //    "greeting did nont contain name, value was `{}`",
        //    result
        //)
    }
}
