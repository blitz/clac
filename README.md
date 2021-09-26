# CLAC - A Reverse Polish Calculator

[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)
![GitHub](https://img.shields.io/github/license/blitz/clac.svg)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/blitz/clac)

Clac is a reverse-polish calculator _like_
[dc](https://en.wikipedia.org/wiki/Dc_(computer_program)), but vastly
simpler and with display of the current argument stack.

## Installing (Nix)

```sh
% nix-env -f ./nix/release.nix -iA clac
```

## Installing (Cargo)

```sh
% cargo build
% cargo install
```

## Example

```sh
% clac
 | 1 1 +
2 | 10 <<
2048 | hex
0x800 | 0x10 |
0x810 | 
```
