use std::mem::drop;
use std::ops::Deref;
// Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// This is not same as Box<>!!
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[derive(Debug)]
enum List {
    // By boxing list, we limit the size of List with its pointer. It is that simple
    // When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    let x = 5;
    let y = MyBox::new(x); // This is not completely equal to &x. Smart pointers act different from references(&)

    assert_eq!(5, x);
    assert_eq!(5, *y); // What happens behind the scenes => *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    // This works only because Mybox has implemented deref
    hello(&m); // Here Rust applies deref coercion. Otherwise we need to write hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // c.drop(); does not work here. We need to impement std::mem::drop for that purpose.
    // The reason is if we drop c here rust will try to drop it at the end of scope and it will cause double free error!
    drop(c); // This works!
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Here d drops before c (stack order). It is automaticly implemented by Rust.
}
