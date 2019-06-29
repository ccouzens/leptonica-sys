# leptonica-sys

Rust FFI bindings to [Leptonica](http://www.leptonica.org/).

Types and functions generated from the functions listed in
[allheaders.h](https://github.com/DanBloomberg/leptonica/blob/master/src/allheaders.h).

## Building

This links to the C library Leptonica.

On Fedora 30 the build dependencies can be installed by running:

```bash
sudo dnf install leptonica-devel clang
```

On Termux 2019 (Android, Android on Chromebooks) the additional dependencies can be installed by running:

```bash
pkg install libclang leptonica-dev
```
