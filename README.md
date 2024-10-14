# rubiks - A library for turning Rubik's cubes and building training sets for solving them
Disclaimer: The focus is on 3x3x3 Rubik's cubes with the standard American colors.

## Installation

- To add to your own cargo project, add the following line to your Cargo.toml
    - `rubiks = { git = "https://github.com/landonwork/rubiks" }`
- I have yet to publish the crate but I hope to make the library available on crates.io, as well as two binaries:
    - `rubiks` for creating "books" or training sets
    - `cubix` for turning a virtual Rubik's cube in the terminal
<!-- - Run `cargo install rubiks` to install the `rubiks` CLI and the `cubix` TUI -->

## Introduction

`rubiks` is a personal project focusing on efficiently exploring the Rubik's cube space in order to generate
datasets, or "books", for training Rubik's cube solving agents. I'm in the middle of refactoring but here is
the general road map:

### Road Map

#### v0.1

- [x] There is a cube
- [x] You can turn the cube
- [x] You can save and load a "store" that has cube-depth pairs

#### v0.2

- [x] Moves, Turns, and QuarterTurns are all valid actions
- [ ] Improved book creation (inspired by Matt Macauley's concept of the "Big Book" which contains every Rubik's cube state
  and the best move to solve the cube from that state - I highly recommend watching his "Visual Group Theory" series on YouTube!)
- [ ] Improved performance in book creation and memory usage
- [x]  [Words](https://en.wikipedia.org/wiki/Word_(group_theory)) which can be reduced and converted to normal form

#### v0.3

- [ ] Library usable from Python
- [ ] Iterable datasets, suitable for training ML models
- [ ] Compatibility with strategic-game-cube dataset
- [ ] Depth inference
- [ ] Sub-word substitution via Books
- [ ] 3D cube visualization in Cubix using Bevy? [NightsWatchGames](https://github.com/NightsWatchGames) has a public Github repo showing a
  [working Rubik's cube](https://github.com/NightsWatchGames/rubiks-cube) made with the Bevy game engine. Perhaps I ask
  to use it or colab??

#### v0.4

- [ ] Cube parity and validation (boolean method for determining the parity and validity of a cube)
    - [ ] corner-edge parity
    - [ ] flipped-edge parity
    - [ ] corner-twist parity
- [ ] TUI improvements
- [ ] Python interface improvements
- [ ] 20th cubelet inference?

#### v0.5

- [ ] Conjugacy sets?
- [ ] 3D cubix interface?
- [ ] Algorithm sandbox in Cubix?
