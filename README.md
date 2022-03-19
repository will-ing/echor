# echor

---

## Searches file using CLI

The CLI app that mimics echo, but written in rust

You have the option -n to omit newline, To use in bash enter:

```terminal
cargo build 
/target/debug/echor -n hello world

or with cargo

cargo run -- -n hello world
```

To export output to file:

```terminal
cargo run -- -n hello world > file.txt
```

---

## Tools Used

- VS Code
- Rust
- clap
- assert_cmd
- predicates

---

## Recent Updates

- Created 2022-03-19

---

## Getting Started

Clone this repository to your local machine.

```terminal
git clone https://github.com/will-ing/echor.git
```

```terminal
cd YourRepo/YourProject
cargo run word sample.txt
```

Unit testing is included in the project

```terminal
cargo test
```

---

| Command | Description |
| ---- |---- |
| du -shc | Get the size of the current directory
| cargo run -- -n hello world | The `--` to seperate the flag is not for cargo.
| cargo run 1>out 2>err | `>` Sends the output to a file |
| -q, --quiet | Print the output only |
| -n | omit_newline |

---

## Change Log

***The change log will list any changes made to the code base.***
1.0 App created

---

## Authors

Will Koger\
March 2022

## References

Rust book\
https://www.youtube.com/c/LetsGetRusty/videos
