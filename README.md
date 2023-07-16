# Rusjure

Rusjure is a compiler and a programming language highly inspired by Clojure, written in Rust.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### Project structure

The project is divided into apps (binaries) and libs (shared code).

#### apps/compiler

This is the binary of the compiler itself. Depends on some crates from libs.

#### libs/corelib

The library that is linked to the generated LLVM code.
Linked even for the JIT compilation.

#### libs/lexer

Takes string and creates the token tree.

Uses [Pest](https://pest.rs/) library, which allows to do so via a definition of grammar (the PEG format).

#### libs/parser

The purpose of this crate is to take the token tree and JIT-it, then produce a structure to be compile down to binary.

#### libs/tokens

Contains just the definition of the tokens, the token tree and the token stream.

## License

Rusjure and all it's parts, unless specified otherwise, is licensed under the MIT and the Apache licenses.
See the [MIT license file](LICENSE-MIT.txt) and the [Apache license file](LICENSE-APACHE.txt) for more information.
