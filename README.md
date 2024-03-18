# Lagrangian Interpolation

This is a technique that interpolates a function from tabulated values. An arbitrary number of tabulated values may be used, and they may
have any arbitrary spacing. This program includes Point and Table `struct`s so we can make use of the function, but in general, it would
be best to modify this source to suit a larger program, and drop it in as a routine.

## Usage

### With `gnuplot`

Do
```shell
cargo run > output.dat
```
to run the program, load the points to a file, and 
```shell
cargo test
```
to run the test(s)! Easy!

To plot the data with `gnuplot`, do the following:
```shell
gnuplot lagrangian_interp.plt > plot.png
```
to store the output in a .png file. If you want another format, read gnuplot's docs! They're pretty detailed.

### With `plotly`

First, checkout the branch "plotly" like so:
```shell
git switch plotly
```
and then do
```shell
cargo run
```
and it will open an interactive graph in your default browser/webview. This makes use of the [plotly](https://docs.rs/plotly) crate,
which interfaces with the `plotly.js` library, and gives Rust Jupyter-Notebook-level data visualization capabilities. Check it out!

## Credit

The implementation was guided by [this](https://phys.libretexts.org/@go/page/8091?pdf) excellent textbook on Celestial Mechanics.
Eventually I hope to have a program that can plot an entire ephemeris from initial conditions, but it turns out orbital mechanics
is hard, so don't hold your breath.

You may have the source of this particular crate under the terms of the BSD 2-clause license, or any similar no-strings-attached OSS license.
