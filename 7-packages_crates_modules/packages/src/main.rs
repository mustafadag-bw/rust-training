// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable. A crate is the smallest amount of code that the Rust compiler considers at a time.
// A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.
// A package can contain as many binary crates as you like, but at most only one library crate.
// A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
// Modules and use: Let you control the organization, scope, and privacy of paths
// "use" keyword brings a path into scope; and the "pub" keyword make items public
// Paths: A way of naming an item, such as a struct, function, or module

// Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.

use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;

// use std::io;
// use std::io::Write; => use std::io::{self, Write}; OR use std::collections::* not suggested since it imports everything under collections

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let mut map = HashMap::new();
    map.insert(1, 2);
}
