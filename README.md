### Wc

Partial Rust implementation of the wc command

wc - print newline, word, and byte counts for each file

### Name
wc - print newline, word, and byte counts for each file

### Synopsis
wc [OPTION]... [FILE]...
wc [OPTION]...

### Description

Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, read standard input.

```
-c, --bytes
    print the byte counts

-m, --chars
    print the character counts

-l, --lines
    print the newline counts

-w, --words
    print the word counts
--help
    display this help and exit
--version
    output version information and exit
```