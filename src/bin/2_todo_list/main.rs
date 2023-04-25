mod todos;
// (1) What is this line doing?  How does it relate to the `use todos::State;` line below?

use std::num::ParseIntError;

use crate::todos::State;

enum Action {
    Help,
    Quit,
    List,
    MarkDone(usize),
    Add(String),
    Remove(usize),
}

fn main() {
    let mut state = State::new();
    println!("Welcome to the todo list app. Type help for help.");

    loop {
        let action = get_action();
        match action {
            Ok(Action::Help) => println!(
                "    list, l: list todos
    add, a: add a todo
    remove, r: remove a todo
    mark_done, m: mark a todo as done
    help, h: display this help
    quit, q: quit"
            ),
            Ok(Action::Quit) => break,
            Ok(Action::List) => {
                for todo in state.list() {
                    println!("    {todo}");
                }
            }
            Ok(Action::MarkDone(id)) => {
                match state.mark_done(id) {
                    Some(todo) => println!("    {todo}"),
                    None => println!("    no todo found with id {id}"),
                };
            }
            Ok(Action::Add(text)) => {
                let todo = state.add(text);
                println!("    {}", todo);
            }
            Ok(Action::Remove(id)) => match state.remove(id) {
                Some(todo) => println!("    removed {}", todo.text),
                None => println!("    no todo found with id {id}"),
            },
            Err(error) => println!("Error in input: {error}"),
        }
    }
}

fn get_action() -> Result<Action, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|err| err.to_string())?;
    // (2) Why do we need `map_err` here?
    // (3) Based on the discussion we had about error handling, what
    // alternative ways could we handle this error?

    let mut tokens = tokenize(1, &input);
    // (4) `tokenize` returns an Iterator, which is a trait.  What is a trait?
    // Where is the documentation for the `Iterator` trait?

    match tokens.next() {
        Some("help" | "h") => Ok(Action::Help),
        Some("quit" | "q") => Ok(Action::Quit),
        Some("list" | "l") => Ok(Action::List),
        Some("add" | "a") => {
            let argument = tokens
                .next()
                .ok_or_else(|| "add requires an argument".to_owned())?;
            Ok(Action::Add(argument.to_owned()))
        }
        Some("remove" | "r") => {
            let argument = tokens
                .next()
                .ok_or_else(|| "add requires an argument".to_owned())?;
            Ok(Action::Remove(
                argument
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?,
            ))
        }
        Some("mark_done" | "m") => {
            let argument = tokens
                .next()
                .ok_or_else(|| "add requires an argument".to_owned())?;
            Ok(Action::MarkDone(
                argument
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?,
            ))
        }
        Some(action) => Err(format!("Unknown action: {}", action)),
        None => Err("No action given".to_owned()),
    }
}

/// Converts an input string into n whitespace-sparated tokens and a final "rest" token
///
/// ## Example:
///
/// ```
/// let mut tokens = tokenise(2, "one two three four five");
/// assert_eq!(tokens.next(), Some("one"));
/// assert_eq!(tokens.next(), Some("two"));
/// assert_eq!(tokens.next(), Some("three four five"));
/// assert_eq!(tokens.next(), None);
/// ```
fn tokenize(tokens: usize, input: &str) -> impl Iterator<Item = &str> {
    input
        .splitn(tokens + 1, char::is_whitespace)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
}

// Extensions
// ----------
//
// As mentioned before, the tokenize function returns something that implements the trait
// `Iterator`, but this is just the general interface type.  What is the concrete type that
// will get returned by this function?  How can we find this out?
// Hint: you can do this using your IDE, or by forcing the compiler to reveal its secrets.
//
// ---
//
// Currently we can only mark a Todo as being done.  Extend the `mark_done` action
// so that it takes a second argument, either "true" or "false", to explicitly set the state
// of the Todo.  For example, `mark_done 1 false` would mark the Todo with id 1 as not done.
//
// To implement this, a few changes need to happen:
// 1. The Action::MarkDone variant needs to carry to pieces of information: the id and the new state.
// 2. The `get_action` function needs to be able to parse the second argument.
// 3. The `mark_done` method on `State` needs to take an extra argument.
//
