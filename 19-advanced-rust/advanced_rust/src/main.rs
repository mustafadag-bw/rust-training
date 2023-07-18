use std::{fmt, slice};
use std::ops::Add;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;


// Global variable
// Accessing and modifying mutable static variables is unsafe.
// Different from const, a static variable have a fixed address in memory. Using the value will always access the same data. 
// Const are allowed to duplicate data whenever they used
static _HELLO_WORLD: &str = "Hello, world!";
// ! reading and writing counter is Unsafe due to threads.
static mut COUNTER: u32 = 0;

// Default parameter for Add trait is self ("test" + "fetch" => default &str etc.)
/* trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
} */

/* struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
} */

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}

// This is C implemented absoluet method
extern "C" {
    fn abs(input: i32) -> i32;
}

// This part is used for other programming langugages to call Rust functions
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// function itself is safe although have unsafe part. Reason is due to assert! part.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let mut num = 5;
    let mut numbers = [0,1,2,3,4,5,6,7];
    let mut_slice = &mut numbers[0..6];

    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}, r2 is: {}", *r1, *r2);
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    let my_new_arr = split_at_mut(mut_slice, 5);
    println!("my new arr = {:?}", my_new_arr);
    // my_new_arr dropped ownership of mut_slice hence it is reusable
    println!("my mut arr = {:?}", mut_slice);

    add_to_count(5);
    // mutating static data is UNSAFE!
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    let point_a = Point { x: 1, y: 0 };
    assert_eq!(
        point_a + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let person = Human;
    person.fly();
    Pilot::fly(&Human);
    println!("A baby dog is called a {}. A baby animal is called", <Dog as Animal>::baby_name());
    point_a.outline_print();

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_statuses: Vec<Status> = (0u32..20).map(|par| {
        if par % 2 == 0 {
            Status::Value(par)
        } else {
            Status::Stop
        }
    }).collect();
    println!("The list of statuses: {:?}", list_of_statuses);
    Pancakes::hello_macro();
}
