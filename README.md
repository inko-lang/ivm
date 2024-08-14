# ivm: the Inko version manager

ivm is a version manager for Inko, and makes it easy to install Inko on
different platforms.

ivm is the successor to Inko's previous version manager,
[ienv](https://gitlab.com/inko-lang/ienv). Unlike ienv, ivm works on Linux,
macOS, and FreeBSD.

For more details about ivm, how to use it, and how to install it as a user,
refer to the
[ivm documentation](https://docs.inko-lang.org/manual/latest/getting-started/ivm/).

## Supported platforms

* Linux
* macOS
* FreeBSD

Since Inko requires a 64-bits platform, so does ivm.

## Requirements

* Rust 1.78 or newer

## Installation

You can install ivm using `cargo`:

    cargo install ivm --force

Alternatively, you can build from source:

    git clone git@github.com:inko-lang/ivm.git
    cd ivm
    cargo build --release

You can now move `target/release/ivm` into your PATH.

## License

All source code in this repository is licensed under the Mozilla Public License
version 2.0, unless stated otherwise. A copy of this license can be found in the
file "LICENSE".
