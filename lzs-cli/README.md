# Lempel–Ziv–Storer–Szymanski de-/compression

lzs-cli is a cli interface to the `lzs` crate.

# Installation

```shell
cargo install lzs-cli
```

# Usage

```
lzs <'e'|'d'> <ei,ej,c>
```

Either 'e' or 'd' to en-/decode.
`ei,ej,c` are the compression parameters, see the lzs crate
for more information about that.

Example:
```shell
lzs e 10,4,0x20 <input >outout
```

# Lack of a header

This algorithm has by design no header at all. Please be aware that it is not
possible to check if the contents is correct, or even the length matches.
It is recommended to add a header based on the requirements.

Thus, this may be not a suitable program for you, but it is easy
to create an own program - use this as a starting point.
