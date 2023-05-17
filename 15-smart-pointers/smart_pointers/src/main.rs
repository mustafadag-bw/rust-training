use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::mem::drop;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// Rc<T> => keeps track of number of number of references and provides us possibility to give ownership to multiple parameters! READ ONLY!!!

// RefCell<T> is only for use in single-threaded scenarios and will give a compile-time error if used in a multithreaded context

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at RUNTIME. (Use last carefully)
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

// Rc::downgrade returns Weak<T>. It increases weak_count for referencing and it does not affect whether the value will be dropped (different from clone)
// With this we can avoid reference loops and memory leak. .upgrade returns Option<Rc<T>> and we can use that to determine whether it is dropped.

// Because the value that Weak<T> references might have been dropped, to do anything with the value that a Weak<T> is pointing to, you must make sure
// the value still exists. Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>. You’ll get a result of Some
// if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped. Because upgrade returns an Option<Rc<T>>, Rust
// will ensure that the Some case and the None case are handled, and there won’t be an invalid pointer.


// This is not same as Box<>!!
struct MyBox<T>(T);

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

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

enum DupList {
    Cons(i32, Rc<DupList>),
    Nil,
}

// We can mutate i32 due to RefCell but cannot modify EdgeList by its own (quite complicated)
// Rc<RefCell> is shared and if changed each shared party knows and sees the change
// Refcell<Rc> is not shared except by reference. In order to mutate inner data we need to mutably borrow it from outer Refcell and then access to immutable Rc. Only way to mutate it is replace it to wholly new Rc.
// Rc<Refcell> is much more useful to mutate common params in different objects.
// There is Memory loop possibility in Refcell<Rc>
#[derive(Debug)]
enum EdgeList {
    Cons(Rc<RefCell<i32>>, Rc<EdgeList>),
    Nil,
}

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

    let a = Rc::new(DupList::Cons(
        5,
        Rc::new(DupList::Cons(10, Rc::new(DupList::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    // Rc::clone only duplicates the number of reference. It does not clone all parameters in a!
    let b = DupList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = DupList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2

    let value = Rc::new(RefCell::new(5));

    let q = Rc::new(EdgeList::Cons(Rc::clone(&value), Rc::new(EdgeList::Nil)));
    let p = EdgeList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&q));
    let t = EdgeList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&q));

    *value.borrow_mut() += 10; // borrow_mut returns RefMut<T> smart pointer. We change inner value using deref operator (*)

    // Strong & Weak Reference counts and drop rules
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch: Rc<Node> = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // it is none since branch is dropped
    println!(
        "leaf strong = {}, weak = {}", // 1 and 0 since branch everything related to it is dropped
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
