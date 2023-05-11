// FnOnce applies to closures that can be called once. All closures implement at least this trait,
// because all closures can be called. A closure that moves captured values out of its body will
// only implement FnOnce and none of the other Fn traits, because it can only be called once.

// FnMut applies to closures that don’t move captured values out of their body, but that might
// mutate the captured values. These closures can be called more than once.

// Fn applies to closures that don’t move captured values out of their body and that don’t mutate
// captured values, as well as closures that capture nothing from their environment. These
// closures can be called more than once without mutating their environment, which is important in
// cases such as calling a closure multiple times concurrently.
// Here is the implementation of unwrap_or_else in Option.
// pub fn unwrap_or_else<F>(self, f: F) -> T
// where
//     F: FnOnce() -> T,
// {
//     match self {
//         Some(x) => x,
//         None => f(),
//     }
// }

use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure is similar to function
    let list = vec![1, 2, 3];

    // Iterator has a method called next and returns Some or None. Note that calling next changes list_iter's inner structure.
    // For for loop, it takes ownership of iter and makes it mutable behind the scenes.

    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
    // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.
    // list_iter.sum() => consumes iter. We cannot use list_iter after sum() since it takes ownership and returns sum of elements.
    let mut list_iter = list.iter();

    assert_eq!(list_iter.next(), Some(&1));
    assert_eq!(list_iter.next(), Some(&2));
    assert_eq!(list_iter.next(), Some(&3));
    assert_eq!(list_iter.next(), None);
    println!("Before defining closure: {:?}", list);

    let v1: Vec<i32> = vec![1, 2, 3];
    // If we do not use .collect() the iterator does not work since it is lazy by default.
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

    assert_eq!(v2, vec![2, 3, 4]);

    let only_borrows = |ls: &Vec<i32>| println!("From closure: {:?}", ls);

    println!("Before calling closure: {:?}", list);
    only_borrows(&list);
    println!("After calling closure: {:?}", list);

    let mut list_mut = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list_mut);

    // FnOnce
    let mut borrows_mutably = || list_mut.push(7);
    // We cannot immutably borrow the list_mut here to print since it is already mutably borrowed by closure
    borrows_mutably();
    println!("After calling closure: {:?}", list_mut);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // This does not work since the FnMut cannot carry ownership of value multiple times.
    //  let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // FnMut and do not carry the ownership out of closure.
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);

    // Here is the logic taken from an audio decoder. 
    let buffer: &mut [i32] = &mut [3213];
    let coefficients: [i64; 12] = [321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332];
    let qlp_shift: i16 = 13;

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
