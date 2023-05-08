fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// References are immutable by default!
fn calculate_length(s: &String) -> usize {
    // s is NOT a String, a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// IMPORTANT
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s. Instead we should return s directly
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// The function do not return whole string it only returns start index and length of string slice
// s: &String limits the usage for the function
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("The value of s1 is {}.", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    let s1 = String::from("hello");
    // only the referance is propagated to the change
    let len = calculate_length(&s1); // & means the ownership of s1 is not propagated to the function
                                     // if you have a mutable reference to a value, you can have no other references to that value.
                                     // After lifetime of the first mutable reference ends we can reference it again. (No data race!)
                                     // We cannot have mutable reference if the value is referenced immutably. Reason is the other reference do not expect any change.
    let mut s = String::from("hello");
    //  reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    change(&mut s);
    let hello = &s[0..5];
    let hello_2 = &s[..5]; // Same as above
    let len_s = s.len();
    let world = &s[7..len_s];
    let world_2 = &s[7..];
    println!(
        "Parameters are: {} {} {} {}. length of s is {}",
        hello, hello_2, world, world_2, len_s
    );
    // immutable borrow occurs below
    let first_word_s = first_word(&s);
    // s.clear(); throws an error here since s is immutably borrowed by first_word_s.
    // String literals (below) are also string slices
    let my_string_literal = "hello world";
    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
    let word = first_word(&my_string_literal);

    println!(
        "The length of '{}' is {}. s3 is {} and s is {}. First word of s is {}. Word is {}",
        s1, len, s3, s, first_word_s, word
    );

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
} // Here, x goes out of scope, then s, and s3s. But because s's value was moved, nothing
  // special happens. Also since the ownership of s2 is given to the takes_and_gives_back nothing happens to s2
