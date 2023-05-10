use std::{cmp::PartialOrd, fmt::Display};
use generictypes_traits_lifetimes::{Tweet, Summary, NewsArticle, notify, returns_summarizable};

// we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our crate(generic_...)
// Either trait or the type should be in our crate to implement due to coherence

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointDiff<X1, Y1> {
    x: X1,
    y: Y1,
}

// Using generic types won't make your program run any slower than it would with concrete types.
// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
impl<X1, Y1> PointDiff<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointDiff<X2, Y2>) -> PointDiff<X1, Y2> {
        PointDiff {
            x: self.x,
            y: other.y,
        }
    }
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
// lifetime of returned value is as long as the lifetime of the short lived one.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This struct cannot outlive the reference it gets
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The encompassing function for the chapter:
// We can also write T: Display and delete "where" part
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };
    let t = Point {
        x: "test",
        y: "test2",
    };
    let float = Point { x: 4.0, y: 3.0 };
    println!(
        "p.x = {}, t.x = {}, distance float: {}",
        p.x(),
        t.x(),
        float.distance_from_origin()
    );

    let p1 = PointDiff { x: 5, y: 10.4 };
    let p2 = PointDiff { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    notify(&article);
    let tweet = returns_summarizable();
    println!("{}", tweet.summarize());

    // LIFETIMES:
    let r;            // ---------+-- 'a
                                    //          |
    {                               //          |
        let x = 5;             // -+-- 'b  |
        r = &x;                     //  |       |
        println!("r: {}", r);       //  |       |
    }                               // -+       | // r is also dropped

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // result is not valid anymore even though string 1 is not dropped yet.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let sentence = i.announce_and_return_part("call");
    println!("Excerpt part: {}, announce and return: {}", i.part, sentence);

    let s: &'static str = "I have a static lifetime."; // lives till the end of the program.
    let last = longest_with_an_announcement(s, "type", "the announcement.");
    println!("Last: {}", last);
}
