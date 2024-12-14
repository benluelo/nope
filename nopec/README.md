# `nopec`

`nopec` is the canonical *NopeLang* compiler.

## Installation

`nopec` is available as a flake output from this repository:

```sh
nix run github:benluelo/nope#nopec
```

Alternatively, `nopec` can also be installed with cargo:

```sh
cargo install --locked --git https://github.com/benluelo/nope nopec
```

## Usage

`nopec` is able to both build *NopeLang* scripts and evaluate compiled *NopeVM* bytecode.

To compile a script:

```sh
nopec build input.nope -o input.nope.o
```

To evaluate the compiled bytecode:

```sh
nopec eval input.nope.o
```
