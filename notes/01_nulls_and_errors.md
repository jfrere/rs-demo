---
marp: true
---

# Options and Results

## How to program without null and exceptions

---

## Representing Null - Option

```rust
#[derive(Copy, PartialOrd, Eq, Ord, Debug, Hash)]
#[rustc_diagnostic_item = "Option"]
#[cfg_attr(not(bootstrap), lang = "Option")]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

---

## Representing Null - Option

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

---

## Representing Null - Option

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

```typescript
export type Option<T> =
    | { kind: "none" }
    | { kind: "some", value: T };
```

---

## Representing Null - Option

```rust
pub enum Option<T> {
    None,
    Some(T),
}

let has_value: Option<u32> = Some(123);
let no_value: Option<u32> = None;
```

```typescript
export type Option<T> =
    | { kind: "none" }
    | { kind: "some", value: T };

const has_value: Option<number> = { kind: 'some', value: 123 };
const no_value: Option<number> = { kind: 'none' };
```

---

## Representing Errors - Result

```rust
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
#[rustc_diagnostic_item = "Result"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```

---

## Representing Errors - Result

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## Representing Errors - Result

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```typescript
export type Result<T, E> =
    | { kind: "Ok", value: T }
    | { kind: "Err", error: E };
```

---

## Representing Errors - Result

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

let value: Result<i32, String> = Ok(123);
let error: Result<i32, String> = Err(String::from("operation failed"));
```

```typescript
export type Result<T, E> =
    | { kind: "ok", value: T }
    | { kind: "err", error: E };

const value: Result<number, string> = { kind: 'ok', value: 123 };
const error: Result<number, string> = { kind: "err", error: "operation failed" };
```

---

## Extracting Values

1. Explicitly handle both cases using `match`
    ```rust
    let wrapped: Option<i32> = Some(123);
    let result = match wrapped {
        Some(value) => do_something_with_value(value),
        None => handle_none_case(),
    };
    ```

---

## Extracting Values

1. Explicitly handle both cases using `match`
2. Use `.expect("reason")` or `.unwrap()` and crash on an error
    ```rust
    let wrapped: Option<i32> = Some(123);
    let unwrapped: i32 = wrapped.expect("Cannot be None");
    ```

---

## Extracting Values

1. Explicitly handle both cases using `match`
2. Use `.expect("reason")` or `.unwrap()` and crash on an error
3. (Inside a function) use the try operator (`?`) to return early on error/none
    ```rust
    fn returns_result() -> Result<(), String> {
        let wrapped: Result<i32, String> = Some(123);
        let unwrapped: i32 = wrapped?;
        // do stuff with unwrapped ...
    }
    ```

---

## Extracting Values

1. Explicitly handle both cases using `match`
2. Use `.expect("reason")` or `.unwrap()` and crash on an error
3. (Inside a function) use the try operator (`?`) to return early on error/none
4. Use the result/option combinator functions
    ```rust
    let wrapped: Result<i32, String> = Ok(123);
    let unwrapped: i32 = wrapped
        .map(|value: i32| value + 234)
        .unwrap_or(0);
    ```
