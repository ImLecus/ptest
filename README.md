# ptest
A lightweight password generator and evaluator, made in Rust.

## Create passwords

Use this program in your shell to generate secure passwords:
```shell
ptest generate [length] [flags]
```
Examples:
```shell
ptest generate 50 -luds
;lL-9hcIguOr/'>[AnTUsLwo]<:qUa]Wss`?I<'~W_;uvS1zoi
```
```shell
ptest generate 50 -l
nqrgvkfoflfcuahkygqbcwwovgrsotvfjmccwegyojcujxjvys
```

### Flags available
`-l` -> lowercase characters (a-z)
`-u` -> uppercase characters (A-Z)
`-d` -> digits (0-9)
`-s` -> symbols (!\"#$%&\'()*+,-./:;<=>?@\[\\\]^_\`{|}~)
`-e` -> extended ASCII

## Evaluate passwords (WIP)

## Features

- **Really fast**: uses `rand` with ChaCha20 CSPRNG.
- **Secure**: generate strong and secure passwords.
- **Simple CLI**: it goes directly to the point.
- **Zero alloc**: precalculated char pool.

## Installation

### Compiling the code

Firstly, clone the repository:
```shell
git clone https://github.com/ImLecus/ptest
```

In your terminal, run:
```shell
cd ptest
cargo build
cargo install --path .
```

If the installation was successful, the following command must show the program version:
```shell
ptest --version
```