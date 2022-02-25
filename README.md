# Beil - Binary Export Import tooL
[![MIT license](https://img.shields.io/badge/license-MIT-green?style=flat-square)](./LICENSE)
[![Rust](https://github.com/mtzstngl/beil/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/mtzstngl/beil/actions/workflows/rust.yml)

Beil, allows you to inspect and compare your binarys imports and exports.


## Features
- Compare imports and exports between two executables or libraries.
- Display information about a library, e.g. the PE header of a windows library.
- List imports, exports and dependencies of an executable or library.
- Scan the PATH variable for the dependencies of a library.
- Verify if a library exports everything another library imports.


## Installation
You can compile from source by installing Cargo (Rust's package manager) and installing `beil` using Cargo:
```
cargo install beil
```