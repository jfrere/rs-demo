---
marp: true
---

# Rust for Developers -- Introduction

---

## Why is Rust worth learning?

1. High-level programming constructs

---

* Iterators!
    ```rust
    input
        .lines()
        .map(parse_line_1)
        .map(score_match)
        .sum::<u32>()
    ```
* Async/await!
    ```rust
    let token = evaluate_token(&auth, &request).await?
    next.run(request).await
    ```
* Sum types & type safety!
    ```rust
    enum Commands {
        Migrate,
        AddUser { name: String, password: String },
        AddTask { name: String, participants: Vec<String> },
        // ... etc
    }
    ```

---

## Why is Rust worth learning?

1. High-level programming constructs
2. Low-level speed

---

(from Discord via Amazon, left is Go, right is Rust)
![Alt text](https://d2908q01vomqb2.cloudfront.net/ca3512f4dfa95a03169c5a670a4c91a19b3077b4/2022/02/09/sust-rust-9.png)

https://discord.com/blog/why-discord-is-switching-from-go-to-rust

---

(From Figma, old is Typescript, new is Rust)
![Alt text](https://cdn.sanity.io/images/599r6htc/localized/aef8d9f50f52218911be208167b18dac8966d646-800x458.png?w%3D804%26q%3D75%26fit%3Dmax%26auto%3Dformat%26dpr%3D2)

https://www.figma.com/blog/rust-in-production-at-figma/

---

## Why is Rust worth learning?

1. High-level programming constructs
2. Low-level speed
3. Powerful tooling

---

* `cargo` -> Like `npm` but faster and better
  * `crates.io` -> Like `npmjs.com` but with built-in documentation
* `rustup` -> Like `nvm` but faster and better
* Built-in testing, formatting, doc comments, benchmarks
* Linting via Clippy
* `rust-analyzer` IDE plugin (on par with VSCode's TS integration)

*(N.B. these are all maintained by the Rust organisation - if this isn't enough, there's plenty of third-party options as well!)*

---

## Next Steps

1. 
    ```
    git clone git@github.com:jfrere/rs-demo.git
    # or
    git clone https://github.com/jfrere/rs-demo.git
    ```
2. Open in VSCode
3. Click "Reopen in Container"
4. 
    ```
    cargo --version
    cargo build
    ```

*(Alternatively, go to https://rustup.rs/ to install Rust, ignore the "Reopen in Container" instruction, and install the `rust-analyzer` extension in VSCode.)*
