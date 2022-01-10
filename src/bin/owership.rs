fn main() {
    // s is valid if it comes into scope and until it goes out of scope
    //&str(string literal) vs String
    let mut a = "hello";
    a = "hello?";
    let mut b = String::from("hello");
    b.push_str(", world!");
    println!("{}, {}", a, b);
}
