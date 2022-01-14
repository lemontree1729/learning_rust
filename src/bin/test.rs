use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("wrong input!");
    a = a.trim().to_string();
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("wrong input!");
    b = b.trim().to_string();
    let mut temp = String::new();
    while a != temp {
        temp = a.clone();
        a = a.replace(&b, "");
    }
    if a != "" {
        println!("{}", a);
    } else {
        println!("FRULA")
    }
}
