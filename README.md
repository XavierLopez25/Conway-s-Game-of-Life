# Conway's Game of Life in Rust

This project is an implementation of the famous cellular automaton, Conway's Game of Life, realized in Rust. It uses the `minifb` library for graphical visualization and the `rand` crate for random initialization of the patterns on the board.

## Features

- Real-time visualization of the cellular automaton.
- Random initialization of a set of known patterns such as block, beehive, blinker, among others.
- Efficient use of memory and CPU through optimized state update techniques.

## Prerequisites

Before you can run this project, make sure you have Rust and Cargo installed, which come with the [Rustup](https://rustup.rs/) installation or directly with [Rust](https://www.rust-lang.org/). This project has been tested on Rust version 1.79.0, but should be compatible with newer versions.

## Installation

To run this project, clone this repository using Git:

```bash
git clone https://github.com/XavierLopez25/Conway-s-Game-of-Life.git

cd Conway-s-Game-of-Life
```
Then, compile and run the project with Cargo:
```bash
cargo run
```

## Using
Once the program is started, you will see a window with cells that evolve in each cycle of the game. You can experiment with different configurations by editing the code to try different patterns or rules. 

## Exit

To exit the game, you can press the Escape key.

## Contribute
If you are interested in improving this project, your contributions are welcome. Please send your pull requests or create issues to discuss your proposed changes.

## License
This project is licensed under the MIT License - see the LICENSE.md file for details.
