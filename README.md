# ivm: the Inko version manager

ivm is a version manager for Inko, and makes it easy to install Inko on
different platforms.

ivm is the successor to Inko's previous version manager,
[ienv](https://gitlab.com/inko-lang/ienv). Unlike ienv, ivm works on Linux,
macOS, and Windows; and doesn't need a Bash shell.

For more details about ivm, how to use it, and how to install it as a user,
refer to the [documentation](#TODO).

## Requirements

* Rust 1.34 or newer

## Installation

First clone the Git repository:

    git clone git@gitlab.com:inko-lang/ivm.git

Then build it:

    cargo build --release

You can now move `target/release/ivm` into your PATH.

## License

All source code in this repository is licensed under the Mozilla Public License
version 2.0, unless stated otherwise. A copy of this license can be found in the
file "LICENSE".
