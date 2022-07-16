# HEADR (head in Rust)

This program supports the following capabilities:

```shell
headr 1.0.0
sanjayts

USAGE:
    headr [OPTIONS] [FILE]...

ARGS:
    <FILE>...    [default: -]

OPTIONS:
    -c, --bytes <BYTES>    
    -h, --help             Print help information
    -n, --lines <LINES>    [default: 10]
    -V, --version          Print version information
```

# TODO

* Add support for numeric values with suffixes (e.g. 2K)
* Add support for negative counts. So for e.g. -n -100 means print all lines except the last 100 lines

# Reference

https://man7.org/linux/man-pages/man1/head.1.html

# Steps used for creating repo from existing dir

```shell
git remote add origin git@github.com:sanjayts/wcr.git
git reset origin/master
git branch -u origin/master
```
