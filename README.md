wf
==

[![Crates.io](https://img.shields.io/crates/v/wf.svg)](https://crates.io/crates/wf)


A Unix-style command line utility for counting word frequencies.

Usage
-----

`wf [options]`

Reads stdin and outputs newline delimited rows containing each unique word and the number of times it appears, seperated by a space.

Options:
```
-n --nums           Include numbers
-s --sort           Sort output alphabetically by words (incompatible with -f)
-f --freq           Sort output but most to least frequent (incompatible with -s)
-h --help           Display usage information
```

Installation
------------

To install the wf binary, you can now do the following with an up-to-date version of `cargo`:

```
cargo install wf
```
