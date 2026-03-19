// recursive type with Boxes
// use crate::List::{Cons, Nil};
// use std::mem::drop;
use std::ops::Deref;

#[derive(Debug)]
enum _List {
    Cons(i32, Box<_List>),
    Nil,
}

impl<T> Deref for _MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// defining my own smart pointer.
struct _MyBox<T>(T);

struct _CustomSmartPointer {
    data: String,
}

// defining the behavior of Drop
impl Drop for _CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data {}", self.data);
    }
}

impl<T> _MyBox<T> {
    fn _new(x: T) -> _MyBox<T> {
        _MyBox(x)
    }
}

use std::cell::RefCell;

fn main() {
    // let pointer: Box<i32> = Box::new(5);
    // println!("{:?}", pointer);

    // let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // to drop a variable let's say named _a we use the function drop()
    // drop(_a)
    // println!("{:?}", list);
    //
    // RefCell
    let number = RefCell::new(5);
    {
        let mut mutable_value = number.borrow_mut(); // this returns a mutable reference
        *mutable_value += 1;
    }

    println!("number: {}", number.borrow()); // the .borrow
}
