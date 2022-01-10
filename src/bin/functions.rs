fn main() {
    println!("Hello, world!");

    another_function(); // doesn't matter order of declaration
    other_function(5);
    print_labeled_measurement(5, 'h');

    // let x=(let y=6); doesn't work with Rust
    let x = {
        let y = 6;
        y + 1 // if you add semicolon(;) here, it became statement and doesn't return value
    }; // and this is expression right now
    println!("The value of x is: {}", x);
    let a = five();
    println!("The value of a is: {}", a);
}

fn another_function() {
    println!("Another function.");
}

fn other_function(x: i32) {
    println!("The value of x is: {}", x)
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label)
}

fn five() -> i32 {
    5
}
