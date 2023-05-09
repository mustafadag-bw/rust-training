// In C, attempting to read beyond the end of a data structure is undefined behavior.
// You might get whatever is at the location in memory that would correspond to that
// element in the data structure, even though the memory doesn’t belong to that structure.
// This is called a buffer overread and can lead to security vulnerabilities if an attacker
// is able to manipulate the index in such a way as to read data they shouldn’t be allowed
// to that is stored after the data structure.
use dotenv::dotenv;
use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // We return error instead of panicking so that we can propagate the error to the caller
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_easy() -> Result<String, io::Error> {
    // ? syntax gives us opportunity to return the Error over there if there is any.
    // When the ? operator calls the from function, the error type received is converted into the
    // error type defined in the return type of the current function.
    // ? operator can only be used on functions which return Result<T, E> or Option<T>
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_easiest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // This is specific to reading a string from file. Cannot specify the thrown error.
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // Returns None if the result is None
    text.lines().next()?.chars().last()
}

pub struct Guess {
    value: i32,
}
// The conditions in which Guess::new might panic should be discussed in its public-facing API documentation
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // getter of Guess
    pub fn value(&self) -> i32 {
        self.value
    }
}

// main can also return a Result<(), E> instead of ()
// When a main function returns a Result<(), E>, the executable will exit with a value of 0
// if main returns Ok(()) and will exit with a nonzero value if main returns an Err value.
// Executables written in C return integers when they exit: programs that exit successfully
// return the integer 0, and programs that error return some integer other than 0.
// Rust also returns integers from executables to be compatible with this convention.
// The main function may return any types that implement the std::process::Termination
// trait, which contains a function report that returns an ExitCode.
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // RUST_BACKTRACE=1 is set in .env to view full steps of the error on v[99] line || It can also be set as full to view more in detail
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us
    let my_var = env::var("MY_VAR").unwrap_or_default();
    println!("My variable is set to: {}", my_var);
    let v = vec![1, 2, 3];
    // v[99]; // panics!

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    // The vertical bar character (|) is used to delimit the parameters of the closure.
    // If the
    // Same code with more advanced structure:
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    let user_name = read_username_from_file();
    let user_name_easy = read_username_from_file_easy();
    let user_name_easiest = read_username_from_file_easiest();
    let last_char = last_char_of_first_line(&"testts").unwrap_or_default(); // default is ""
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!(
        "user name: {:?}\nuser name: {:?},\nuser name: {:?},\nlast char: {},\nhome: {}",
        user_name, user_name_easy, user_name_easiest, last_char, home
    );
    // ? cannot convert Result to Option or vice versa. In those cases we need to use "ok" on Result "ok_or" on Option

    // We can use Guess Struct for API body to limit incorrect form of data
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");

    println!("Please input your guess.");
    let guess = Guess {
        value: 43,
    };

    println!("You guessed: {:?}", guess.value);

    match guess.value.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }

    Ok(())
}
