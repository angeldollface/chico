# CHICO

![GitHub CI](https://github.com/angeldollface/chico/actions/workflows/rust.yml/badge.svg)

***A tiny library to work with base-10, base-16, and base-2 numbers in Rust!***

## ABOUT

Since I like numbers and I like Rust, I thought I'd port a library that I wrote in [Javascript](https://github.com/angeldollface/zeppo) and [Dart](https://github.com/angeldollface/harpo) to Rust. These three projects all have one thing in common: They all provide simple methods to convert between numbers of different bases. These bases are 2, 10, and 16. Enjoy! *Nota bene: I like the Marx brothers.*

## INSTALLATION

### FOR RUST PROJECTS

To use ***Chico*** in your Rust project, add this line to your project's `Cargo.toml`:

```TOML
chico = "0.3.0"
```

### FOR THE COMMAND LINE

You can install ***Chico*** via Cargo itself using this command:

```bash
cargo install chico
```

Alternatively, you can download a compiled binary for 64-bit systems from this repository's [Releases](https://github.com/angeldollface/chico/releases) section.

## USAGE

To understand how to use *Chico* in your project, please refer to the project's [documentation](https://docs.rs/chico/0.3.0).

## CHANGELOG

### Version 0.1.0

- Initial release.
- Upload to GitHub.

### Version 0.2.0

- Added better handling of errors.
- Added a CLI tool.
- Updated documentation.

### Version 0.3.0

- Made every terser and more elegant.
- Relicensed the project under the [DSL v1](https://github.com/angeldollface/doll-software-license).
- Added support for base-8 numbers in the library.
- Added support for base-8 numbers in the CLI.
- Added `clippy` linting to the CI pipeline.
- Refactored methods for base-2, base-8, and base-16.
- Cycled errors up to the `main` function for the CLI app.
- Removed the `utils` module.

## NOTE

- *Chico* by Alexander Abraham a.k.a. *"Angel Dollface"*
- Licensed under the [DSL v1](https://github.com/angeldollface/doll-software-license).