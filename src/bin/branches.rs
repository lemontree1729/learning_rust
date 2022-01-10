fn main() {
    //---------------- Contorl Flow
    // if Expressions
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 }; // block should be statement(without ;)
    println!("The value of number is: {}", number);

    // Repeating with loop
    let mut count = 0;
    'counting_up: loop {
        // naming loop as counting_up
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // this will break outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    // loop with returning values
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns counter*2
        }
    };
    println!("The result is {}", result);

    // while(conditional loop, used for nesting loop, if, else and break)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // for(looping through a collection, used for safety with out of bounds)
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // most commonly used loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
