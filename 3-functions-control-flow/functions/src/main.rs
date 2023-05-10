fn five() -> i32 {
    5
}

// Statements are instructions that perform some action and do not return a value. (let x = 6 do not return a value)
// Expressions evaluate to a resultant value.
// Rust is expression-based language
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    // This curly brackets define an expression
    let y = {
        let x = 3;
        x + 1 // No semicolomn to return that value
    };

    println!("The value of y is: {y}");
}

fn main() {
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {x}");
}
