# Pesto

---

![](./assets/banner.png)

# What is it?
Pesto is a cli tool that allows you to understand a codebase by querying the comments inside it via tags.

# Usage

- Wrap the piece of code you wish to comment with a tag. Opening tags are defined as **":{some string}"** and closing tags as **"{some string}:"**

```rust
  // :test1
  fn main() {
    println!("Hello World!");
  }
  // test1:
```

- Create a **".pesto"** file in the root of your project and write comments for the code previously tagged

```
  :test1
    Just a test
  test1:
```
- Use the pesto command with a tag to associate the comment with the code

```sh
> pesto test1

notes.pesto
2:      Just a test

./src/main.rs
2:fn main() {
3:    println!("Hello, world!");
4:}
5://
```

![](./assets/usage.mp4)

--- 

# Installation

```sh
  cargo install pesto  
```
---

# Build
```sh
# Build it like a normal rust project
cargo build --release

# Move it to your bins
sudo cp ./target/release/pesto /usr/local/bin/
``
