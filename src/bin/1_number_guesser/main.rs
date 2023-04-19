use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // (1) Why is the exclamation mark in this line necessary?
    println!("Guess the number...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        // (2) What is the purpose of creating an empty `String` here?
        let mut guess = String::new();

        // (3) What happens if the program is run with a bad stdin?
        // (e.g. `cargo run < /tmp`, where /tmp is a directory)
        // Which function is causing this to happen?
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // (4) The `parse` function is generic, and can be used to parse
        // a string into many different types.  How does Rust know here which
        // type convert the string to?
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // (5) What happens if we remove one of these cases from the match statement?
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// EXT
//
// The first line in the program is
//    use rand::Rng;
// but the item `Rng` is not used again in the whole program.
// Why do we need this line?  What happens when it is removed?
//
// ---
//
// There is a type annotation on line 27.
// 1. What other values could this type annotation have that would
//    make sense here?  Where in the Rust documentation might we be
//    able to find a list of these types?
// 2. In line 9, there is no type annotation, but it should be possible
//    to see the inferred type of the variable using your IDE.
//    What happens to this type as you change the annotation on line 27?
//
// ---
//
// This program uses the `loop` construct (line 11).  This is not the only
// way to loop in Rust.  What are the other ways, and why do we use `loop` here?
//
// ---
//
// Line 15 create a `String` buffer to store the user's input.  We need to create
// a buffer somewhere because the incoming bytes need to go somewhere, but is this
// the optimal place for this buffer?  Why/why not?
