mod todos;
// (1) What is the purpose of this line?  What happens if it is removed?

use std::num::ParseIntError;

use todos::State;

enum Action {
    Help,
    Quit,
    List,
    MarkDone(usize),
    Add(String),
    Remove(usize),
}

fn get_action() -> Result<Action, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|err| err.to_string())?;
    // (2) What is the question mark doing in this line?  What happens if it is
    // removed?  What happens if it is replaced with `unwrap`?

    let mut input = input
        .splitn(2, char::is_whitespace)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string());

    match input.next().as_deref() {
        // In the following lines, the strings we're matching on are
        // wrapped with `Some(...)`.  Why is this necessary here?
        // Is there a way to remove the `Some(...)` parts and just have the strings?
        Some("help") | Some("h") => Ok(Action::Help),
        Some("quit") | Some("q") => Ok(Action::Quit),
        Some("list") | Some("l") => Ok(Action::List),
        Some("add") | Some("a") => {
            let argument = input
                .next()
                .ok_or_else(|| "add requires an argument".to_owned())?;
            Ok(Action::Add(argument))
        }
        Some("remove") | Some("r") => {
            let argument = input
                .next()
                .ok_or_else(|| "add requires an argument".to_owned())?;
            Ok(Action::Remove(
                argument
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?,
            ))
        }
        Some("mark_done") | Some("m") => {
            let argument = input
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
                    println!("    {}", todo);
                }
            }
            Ok(Action::MarkDone(id)) => {
                let todo = state.mark_done(id);
                println!("    {}", todo);
            }
            Ok(Action::Add(text)) => {
                let todo = state.add(text);
                println!("    {}", todo);
            }
            Ok(Action::Remove(id)) => {
                let todo = state.remove(id);
                println!("    removed {}", todo.text);
            }
            Err(error) => println!("Error in input: {}", error),
        }
    }
}

// Extensions
// ----------
//
// 1. Currently we can only mark a Todo as being done.  We can extend the `mark_done` action
//    so that it takes a second argument, either "true" or "false", to explicitly set the state
//    of the Todo.  For example, `mark_done 1 false` would mark the Todo with id 1 as not done.
//    To implement this, we need to make a few changes:
//    1. The Action::MarkDone variant needs to carry to pieces of information: the id and the new state.
//    2. The `get_action` function needs to be able to parse the second argument.
//    3. The `mark_done` method on `State` needs to take an extra argument.
//
// ---
//
//
