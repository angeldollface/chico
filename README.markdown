# CHICO :clown_face: :blond_haired_person:

![GitHub CI](https://github.com/angeldollface/chico/actions/workflows/rust.yml/badge.svg)

***A tiny library to work with base-10, base-16, and base-2 numbers in Rust! :clown_face: :blond_haired_person:***

## ABOUT :books:

Since I like numbers and I like Rust, I thought I'd port a library that I wrote in [Javascript](https://github.com/angeldollface/zeppo) and [Dart](https://github.com/angeldollface/harpo) to Rust. These three projects all have one thing in common: They all provide simple methods to convert between numbers of different bases. These bases are 2, 10, and 16. Enjoy! :heart_on_fire: *Nota bene: I like the Marx brothers.*

## INSTALLATION :inbox_tray:

To use *Chico* in your projects, add this to your `Cargo.toml`:

```TOML
[dependencies]
chico = { git = "https://github.com/angeldollface/chico", branch = "main" }
```

## USAGE :hammer:

To import *Chico*'s API, put this line of code inside your Rust code:

```Rust
use chico::*;
```

To refer to *Chico*'s API, peruse the [`src`](src) directory.
To get some more concrete usage examples of *Chico*'s methods,
peruse this library's [unit tests](./src/modules/tests.rs).

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Chico :clown_face: :blond_haired_person:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
