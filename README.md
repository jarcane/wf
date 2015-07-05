wf
==

A Unix-style command line utility for counting word frequencies.

Usage
-----

`wf [options]`

Reads stdin as a string, processes the word count, and outputs newline delimited rows containing first the word, then the number of times it appears.

Options:
```
-n --nums           Include numbers
-s --sort           Sort output alphabetically by words (incompatible with -f)
-f --freq           Sort output but most to least frequent (incompatible with -s)
-h --help           Display usage information
```
