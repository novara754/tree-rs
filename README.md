# tree

A program to recursively list the contents of a folder in a tree-like manner.

## Building

**Requirements:** A working installing of [Rust](https://rust-lang.org/).

To build the project, simply run `cargo build --release` in this directory. Afterwards
the executable can be found under `./target/release/tree(.exe)`.

## Running

You can information about the command line options using the `-h` option:
```
$ tree -h
tree 0.1.0
A program to recursively list the contents of a folder in a tree-like manner.

USAGE:
    tree.exe [OPTIONS] <ROOT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --max-depth <max-depth>    Maximum depth to recurse to. 0 means no recursion

ARGS:
    <ROOT>    The directory from which to start traversing
```

Example call:
```
$ tree ./test -d=1
./test
|---a
|   |---x
|   `---y
|---b
|   |---q
|   |---t
|   `---y
`---c
    |---p
    |---q
    |---r
    `---s
```

## License

Licensed under the [MIT License](LICENSE.md).
