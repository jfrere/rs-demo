---
marp: true
---

# Ownership

---

## Core Ideas

1. Data always needs to live somewhere in a Rust program
2. The data can be permanently moved around, but also temporarily shared
3. The compiler can track this, and will raise an error if the sharing rules aren't followed

---

## Where Can Data Live?

```rust
// the string data lives in this variable
let my_string = "my string data".to_string();
```

```rust
let my_object = MyObject {
    integer_field: 123,
    // this vector (resizable/dynamic array) lives in this field of the MyObject struct
    // the MyObject struct lives in turn in the `my_object` variable.
    vector_field: vec![1, 2, 3],
}
```

```rust
fn my_function(param: Vec<u8>) {
    // in the parameter `param` lives a vector of 8-bit unsigned integers
}
```

---

## What Can We Do With Data?

1. Moving data
2. Dropping data
3. Borrow data immutabily (shared references)
4. Borrow data mutably (unique references)
5. Clone and copy data

---

## Clone & Copy

```rust
let string = "my string data".to_string();
let new_string: String = string.clone();
println!("{:?} {:?}", new_string, string);
```
```rust
let integer = 1000_u32;
let new_integer: u32 = integer;
println!("{:?} {:?}", new_integer, integer);
```

---

## String Types

* `String`
* `&String` -> reference to `String`
* `&str` -> reference to `str`???
* `str` -> ?????

---

## String Types

```rust
fn first_chars(string: &String, count: usize) -> ? {
    // returns the first `count` characters of `string`
    // but what should the return type look like?
}
```

---

## String Types

```rust
struct String {
    // Pointer to the start of a block of memory containing u8 values.
    // This struct owns this memory - therefore all the ownership rules
    // that apply to `String` also apply to this memory.
    memory: *u8,
    // Length of the memory.
    capacity: usize,
    // Size of the string within memory (must be <= capacity).
    size: usize,
}
```

```rust
struct &str {
    // Pointer to the start of a block of memory containing u8 values.
    // This is just a reference to a `String` that owns the data.
    memory: *u8,
    // The length of the slice of string that we're accessing.
    size: usize,
}
```
