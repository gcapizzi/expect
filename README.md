# expect

An assertions/expectations library for Rust. `expect` provides a syntax
inspired by [RSpec](https://rspec.info/documentation/3.9/rspec-expectations/)
and [Gomega](https://onsi.github.io/gomega/), and a set of powerful built-in
matchers. 

## Getting `expect`

`expect` isn't ready for general use just yet, but you can try it by consuming
this Git repository in your `Cargo.toml`:

```toml
[dev-dependencies]
expect = { git = "https://github.com/gcapizzi/expect.git" }
```

## Using `expect`

The basic structure of an expectation is:

```rust
expect(&actual).to(matcher(expected));
expect(&actual).not_to(matcher(expected));
```

For example:

```rust
expect(&(2 + 2)).to(equal(4))
```

## Built-in matchers

### Core matchers

* **`equal`**:
  ```rust
  expect(&"foo").to(equal("foo"))
  ```

### String matchers

* **`match_regex`**:
  ```rust
  expect(&"abc-123").to(match_regex(r"\d{3}"));
  ```

### Collection matchers

* **`contain`**:
  ```rust
  expect(&[1, 2, 3]).to(contain(2));
  expect(&vec![1, 2, 3]).not_to(contain(4));
  ```

### `Option` matchers

* **`be_some`**:
  ```rust
  expect(&Some("foo")).to(be_some());
  ```
* **`be_none`**:
  ```rust
  expect(&None::<&str>).to(be_none());
  ```

### `Result` matchers

* **`be_ok`**:
  ```rust
  expect(&Ok("foo")).to(be_ok());
  ```
* **`be_err`**:
  ```rust
  expect(&Err("foo")).to(be_err());
  ```

### Path matchers

* **`exist`**:
  ```rust
  expect(&env!("CARGO_HOME")).to(exist());
  ```
