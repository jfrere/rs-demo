# Todo List

## Tasks

1. In `main.rs`, there are two lines that indicate that this file references/imports another file:
    ```rust
    mod todos;
    use crate::todos::State;
    ```
    What is the purpose of each of these two lines?
2. In the function `get_action()` in `main.rs`, when we're reading from `stdin`, we use the `map_err()` method.
    1. Why do we need the `map_err` call here?  (I.e. what would happen if we left it out and just kept the question mark?)
    2. What other ways could we handle this error?
3. In `todos.rs`, we create a struct `Todo`.  Above the struct declaration, there is an annotation that looks like this:
    ```rust
    #[derive(Debug, Clone, PartialEq, Eq)]
    ```
    1. What does this `derive` attribute do, and why is it useful in situations like this?
    2. The names in brackets (`Debug`, `Clone`, `PartialEq`, `Eq`) are traits -- roughly equivalent to interfaces in other languages.  What do these traits do?
4. In `todos.rs`, there are two blocks of code that start with the keyword `impl`:
    ```rust
    impl State {
        /* ... */
    }
    
    impl Display for Todo {
        /* ... */
    }
    ```
    1. `State` and `Todo` are two structs defined in this file.  What is `Display`?
    2. The impl blocks define (mostly) methods for a given struct.  Try adding a new method in each of these blocks (e.g. by copying one of the other functions and changing the name).  What happens?
    3. Given the answers to the above two questions, what is the difference between the two types of impl block?  (I.e. between `impl X` and `impl Y for X`)
5. In `todos.rs` in the `impl State` block, some of the functions have a `self` (or `&self` or similar) parameter and others don't.  By looking at how these functions are used, what is the `self` paramter doing?
6. In `todos.rs` there is a block of inline tests.  (This is a common pattern in Rust code.)  How can we run the tests?

## Extensions

1. Question 3 introduces some common traits.  Another trait used in this program is the `Iterator` trait used in the `tokenize` function in `main.rs`.  This provides lazy iteration for sequences and collections.
 
   The `tokenize` function doesn't specify a concrete return type, it just uses the `impl Iterator` syntax.  (`impl Type` in place of an actual type name typically just means "a type implementing this trait will appear in this place".)  But it still has to return something -- what is concrete type that is being returned by this function?  (Hint, try hovering over the parentheses in the `.map` call, or replacing the return type of the function with an incorrect type like `()` or `i32`.)

   What does this type represent?

2. Tests involve writing the word "test" out a lot of times:
   * `#[cfg(test)]`
   * `mod tests {...}`
   * `#[test]`
  What is the purpose of each of these lines?  And how else might we use the `#[cfg(...)]` annotation or `mod` keyword in rust?

## Writing Rust Code

1. Add a test for the `mark_done` method and run it.
2. Currently, we can only mark a Todo as being done.  Extend the `mark_done` action so that it can take a second argument, either `true` or `false`, to explicitly set the state of the Todo.  For example, `mark_done 1 false` would mark the Todo with the id 1 as not done.
   
   To implement this, a few changes need to be made:
   1. The `Action::MarkDone` variant needs to carry an extra piece of information: the new state
   2. The `get_action` function needs to parse the second argument.
   3. The `mark_done` method on `State` needs to take an extra argument.
   
   What changes would need to be made if the second argument were optional?  (I.e. if `mark_done 1` and `mark_done 1 false` were both allowed.)
