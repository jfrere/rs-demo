# Time Tracker

## Tasks

1. This project has some third-party dependencies.
    * Where are the dependencies defined?  What tool is used to manage dependencies?
    * What is the purpose of each of these dependencies?
        * `clap`
        * `serde`/`serde_json`
        * `anyhow`
2. In [the previous exercise](../2_todo_list/todos.rs), we used `map_err` to convert an error from one type to another, so we could return it from a function.  In [entries.rs](entries.rs), we don't use `map_err`, and instead there is a `with_context` function.  Why does this work?  (Hint: this relates to the `anyhow` library.)

## Writing Rust Code:
1. Get the program in a working state.
    * It should be possible to show a list of what was done for a given day or time period
    * It should be possible to add new entries (for today or for other days) to that list
    * It should be possible to delete and/or modify entries that exist
    * The list should be saved between runs (this has been mostly implemented already)
