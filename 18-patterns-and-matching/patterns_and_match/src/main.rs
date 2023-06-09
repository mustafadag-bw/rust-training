struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // Here we cannot put &&  age > 30 since the age is still covered by Result. We uncover it in the inner scope
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = Some(5);
    let y = 10;

    match x {
        // | works as or 1..=5 => covers range from 1 to 5 including both
        Some(1 | 2) => println!("Got 50"),
        // y here is matched arms of x so this prints "Matched, y = 5". We can use Some(n) if n == y to use y in match
        Some(y) /*if n == y */ => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let my_char = 'c';

    match my_char {
        'a'..='j' => println!("early ASCII letter {my_char}"),
        'k'..='z' => println!("late ASCII letter {my_char}"),
        _ => println!("something else"),
    }

    // Destructing
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructing for nested structs
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // Nested struct
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}, inches: {inches} x: {x}, y: {y}");

    let s = Some(String::from("Hello!"));
     // does not bind the s hence s still has the ownership of the string.
    if let Some(_) = s {
        println!("found a string");
    }
    // We can use .. to ignore rest of the parameters in a struct or tuple

    println!("{:?}", s);

    let num = Some(4);

    match num {
        // More conditioning
        // The downside is that the compiler doesn't try to check for exhaustiveness when match guard expressions are involved.
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    enum EnumMessage {
        Hello { id: i32 },
    }

    let msg = EnumMessage::Hello { id: 5 };

    // @ is used to check whether a parameter is inside a range
    match msg {
        EnumMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        EnumMessage::Hello { id: 10..=12 } => { // Cannot use value of id here
            println!("Found an id in another range")
        }
        EnumMessage::Hello { id } => println!("Found some other id: {}", id),
    }

}
