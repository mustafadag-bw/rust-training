// SUMMARY: Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module.
// The compiler knows to look in front_of_house because it came across the module declaration in the crate root.
mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::{hosting, serving};
pub use crate::back_of_house::cooking;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_payment();
    cooking::spagetti();
}