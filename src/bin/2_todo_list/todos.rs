use std::fmt::{self, Display};

#[derive(Debug, Default)]
pub struct State {
    last_id: usize,
    todos: Vec<Todo>,
}

impl State {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            todos: Vec::new(),
        }
    }

    pub fn list(&self) -> &[Todo] {
        &self.todos
    }

    pub fn add(&mut self, todo: String) -> &Todo {
        self.last_id += 1;
        let todo = Todo {
            id: self.last_id,
            text: todo,
            done: false,
        };
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    pub fn remove(&mut self, id: usize) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id == id)?;
        Some(self.todos.remove(index))
    }

    pub fn mark_done(&mut self, id: usize) -> Option<&Todo> {
        let index = self.todos.iter().position(|todo| todo.id == id)?;
        self.todos[index].done = true;
        Some(&self.todos[index])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// (5) What does the `derive` attribute do?
// What do the `Debug`, `Clone`, `PartialEq`, and `Eq` traits do?
// What is the difference between PartialEq and Eq?
pub struct Todo {
    id: usize,
    pub text: String,
    pub done: bool,
}

impl Display for Todo {
    // (6) On line 9, we have a different block that starts with the keyword `impl`.
    // What is the difference between this block and that one?  And what is `Display`?

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let done = if self.done { "x" } else { " " };
        write!(f, "[{}] {} {}", done, self.id, self.text)
    }
}

// (7) Here we have some inline module tests.
// How can we run them?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_returns_a_list_of_empty_todos() {
        let state = State::new();
        assert_eq!(state.list(), &[]);
    }

    #[test]
    fn test_adding_todo_updates_list_of_todos() {
        let mut state = State::new();
        state.add("Buy milk".to_owned());
        assert_eq!(
            state.list(),
            &[Todo {
                id: 1,
                text: "Buy milk".to_owned(),
                done: false
            }]
        );
    }

    #[test]
    fn test_can_remove_todo_from_todo_list() {
        let mut state = State::new();
        state.add("Buy milk".to_owned());
        state.add("Buy eggs".to_owned());
        state.add("Buy bread".to_owned());
        let todo = state.remove(2);
        assert_eq!(
            todo,
            Some(Todo {
                id: 2,
                text: "Buy eggs".to_owned(),
                done: false
            })
        );
        assert_eq!(
            state.list(),
            &[
                Todo {
                    id: 1,
                    text: "Buy milk".to_owned(),
                    done: false
                },
                Todo {
                    id: 3,
                    text: "Buy bread".to_owned(),
                    done: false
                }
            ]
        );
    }
}

// Extensions
// ----------
//
// 1. Test the `mark_done` method.
// 2. Tests involve writing the word "test" out a lot of times!
//    * `#[cfg(test)]`
//    * `mod tests { ... }`
//    * `#[test]`
//    What is the purpose of each of these lines?  What are they telling the compiler?
//
// ---
//
// The function signatures of `add(...)` and `mark_done(...)` both return the type `&Task`,
// but the function signature of `remove(...)` returns the type `Task`.  What is the difference
// between the two types, and why does `remove(...)` have to return `Task` instead of `&Task`?
//
// ---
//
// What is the difference between the keyword `mod` as used in this file, and `mod` as used in `main.rs`?
// (And also what are the similarities?)
//
