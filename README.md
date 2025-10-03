# rrdbc

[![GitHub repo](https://img.shields.io/badge/github-oxibus/rrdbc-8da0cb?logo=github)](https://github.com/oxibus/rrdbc)
[![crates.io version](https://img.shields.io/crates/v/rrdbc)](https://crates.io/crates/rrdbc)
[![crate usage](https://img.shields.io/crates/d/rrdbc)](https://crates.io/crates/rrdbc)
[![docs.rs status](https://img.shields.io/docsrs/rrdbc)](https://docs.rs/rrdbc)
[![crates.io license](https://img.shields.io/crates/l/rrdbc)](https://github.com/oxibus/rrdbc)
[![CI build status](https://github.com/oxibus/rrdbc/actions/workflows/ci.yml/badge.svg)](https://github.com/oxibus/rrdbc/actions)
[![Codecov](https://img.shields.io/codecov/c/github/oxibus/rrdbc)](https://app.codecov.io/gh/oxibus/rrdbc)

## dbc2json

Convert dbc file to json file.

```sh
$ ./dbc2json -h
dbc2json 0.1.0
Convert DBC file to JSON

USAGE:
    dbc2json --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input dbc file
    -o, --output <output>    Output json file
````

usage example:

```sh
dbc2json -i input.dbc -o output.json
```

## Resources

* <https://bitbucket.org/tobylorenz/vector_dbc/src/master/>
* <https://github.com/xR3b0rn/dbcppp>
* <https://github.com/cantools/cantools>

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
