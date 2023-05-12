use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}! {num} plus random is {}", add_one::add_one(num), add_one::add_random(num));
}
