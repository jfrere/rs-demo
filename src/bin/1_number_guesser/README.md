# Number Guesser

## Tasks

1. Run the code!  Use `cargo run --bin 1_number_guesser` to compile the code and run it.
2. At the start of the `main()` function, there is a line like:
   ```rust
   println!("...");
   ```
   What does `println` do, and why is the exclamation mark necessary?  (Hint: try removing the exclamation mark and re-running `cargo run`.)
3. At the end of the `main()` function, there is a match statement.  What happens if one of the arms of the match statement is removed?
4. On line 13, we declare a variable `guess`, which we later use as a buffer to read data into.  We declare this variable as `mut` which stands for "mutable".  What would happen if we had used `let` instead of `let mut`?  (Hint: again, the compiler or your IDE can help us here.)
5. Try running the code again, but passing bad stdin to the program (e.g. `cargo run --bin 1_number_guesser < /tmp` passes a directory instead of a file).  This causes an error: on what line of the code is the error happening?
6. As mentioned before, on line 13, we declare the variable `guess`, which is a string buffer.  But on line 19, we declare another variable `guess` with a different type.  Why is this allowed?

## Extensions

1. `cargo run` runs the program, automatically recompiling it if we've made changes.  What other commands are there?  Specifically, how might we do the following things:
   1. Let `cargo` run the compiler over our code to check if it's correct or not, without compiling anything.
   2. Run the compiler and build a _release_ artefact.  (Bonus: where can the release binary be found?)
   3. Check our code against a linter to catch common errors and issues.

2. There is a written type annotation on line 25 (`: u32`).  In most cases, the types of variables can automatically be inferred, but `parse` can return multiple different types, and it's not clear which one we want here without the type annotation.
   
   Try playing around with different types here (e.g. other integer types like `u8` or `i64`, other basic types like `bool` or `f32`, or other types altogether like `IpAddr`).  In VSCode, by default, the Rust extension shows "inlay hints" in places where types have been inferred -- what happens to these hints (e.g. on line 9 and line 27) as you change the types?  What sort of errors happen?

3. We use the `loop` construct as the main loop of the program, but there are three ways of doing loops in Rust.
   1. What are they?
   2. Why might `loop` be preferred in this case?  (Hint: try replacing `loop` with `while(true)` and re-running the compiler.)