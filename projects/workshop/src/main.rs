/// # Borrowing

fn a<'a>(other: &'a str) -> &'a str {
    &other[1..3]
}

fn main() {
    let number = 42;
    let text = String::from("Number");

    let text_ref = &text; // create &String reference to String _container_
    println!("{} is {}", text_ref, number); // OK

    // Prints "Number is 42"

    let text_ref = &text[0..3]; // create &str reference to string contents from [0 to 3)
    println!("{} is {}", text_ref, number); // OK

    // Prints "Num is 42"

    let text_ref = &text[..]; // create &str reference to the whole String
    println!("{} is {}", text_ref, number); // OK

    // Prints "Number is 42"

    // We can have as many immutable references as we like
}