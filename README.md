# Lagrangian Interpolation

This is a technique that interpolates a function from tabulated values. An arbitrary number of tabulated values may be used, and they may
have any arbitrary spacing. This program includes Point and Table `struct`s so we can make use of the function, but in general, it would
be best to modify this source to suit a larger program, and drop it in as a routine.

## Usage

Just do
```shell
cargo run
```
to run the program, and 
```shell
cargo test
```
to run the test(s)! Easy!

## Credit

The implementation was guided by [this](https://phys.libretexts.org/@go/page/8091?pdf) excellent textbook on Celestial Mechanics.
Eventually I hope to have a program that can plot an entire ephemeris from initial conditions, but it turns out orbital mechanics
is hard, so don't hold your breath.

You may have the source of this particular crate under the terms of the BSD 2-clause license, or any similar no-strings-attached OSS license.
