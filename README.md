# minigrep

This is a command-line tool implemented in Rust for searching text files for a specific string or regular expression pattern, similar to the Unix command grep.

1. To grep the contents of a file:

```sh
cargo run -- <query> <file_path>
```

2. To grep the stdout of a command:

```sh
cargo run -- <command> <query> 
```
