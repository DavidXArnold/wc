# Wc

Rust implementation of the wc command

wc - print newline, word, and byte counts for each file

Name
wc - print newline, word, and byte counts for each file
Synopsis
wc [OPTION]... [FILE]...
wc [OPTION]... --files0-from=F
Description

Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, read standard input.

-c, --bytes
    print the byte counts
-m, --chars
    print the character counts
-l, --lines
    print the newline counts
--files0-from=F
    read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input
-L, --max-line-length
    print the length of the longest line
-w, --words
    print the word counts
--help
    display this help and exit
--version
    output version information and exit