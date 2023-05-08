// In the User struct definition, we used the owned String type rather than the &str string slice type.
// This is a deliberate choice because we want each instance of this struct to own all of its data
// and for that data to be valid for as long as the entire struct is valid.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// derive Debug is to be able to println! the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This is implementations for Rectangle Struct
impl Rectangle {
    // If we want to change the rectangle parameters we can also get self directly not the reference
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Tuple struct
struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // user 1 is immutable
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    // We cannot use user2 as whole anymore since the string of username of user2 is moved to user3
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    let black = Color(0, 0, 0);

    user2.sign_in_count += 1;
    println!(
        "User 1 username: {}. User 2 activity: {}, email: {}, signin count: {}. User 3 email is: {}. black: {}",
        user1.username, user2.active, user2.email, user2.sign_in_count, user3.email, black.0
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // dbg! trait can help us to track what is going on with our code.
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    if rect1.valid() {
        println!(
            "The rectangle has a nonzero height and width; width: {}, height: {}",
            rect1.width, rect1.height
        );
    }

    // following are the same:
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    let can_hold = rect2.can_hold(&rect1);
    let square = Rectangle::square(3);
    println!(
        "Rect 2 can hold rect 1: {}, square is ,{:?}",
        can_hold, square
    );
}
