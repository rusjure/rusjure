# Rusjure

Rusjure is a compiler and a programming language highly inspired by Clojure, written in Rust.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### Project structure

The project is divided into a few crates: the AST, the parser and the executor.

#### AST

Contains the definition of expressions, terms and basically the code structure.

#### Parser

A library for parsing Rusjure code into an AST.

Uses [Pest](https://pest.rs/).

#### Executor

A library for executing and compiling the AST.

Uses [LLVM](https://llvm.org/).

## License

Rusjure and all it's parts, unless specified otherwise, is licensed under the MIT and the Apache licenses.
See the [MIT license file](LICENSE-MIT.txt) and the [Apache license file](LICENSE-APACHE.txt) for more information.
