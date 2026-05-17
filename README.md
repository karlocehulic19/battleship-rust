# Rust Tetris

Tetris in terminal implemented with Rust and [ratatui library](https://ratatui.rs/)

## Terminal implementation with ratatui

- Using **Text** widget to manage tetris lines with vectors

### Project architecture

Project can be separated into two parts, game(actual game logic), TUI(loader of the game implemented with ratatui)
Game and TUI talk via channels and are run on separated threads. (This lowers coupling and enables other UI's to be implemented)

---

### Block implementation

Each block can be represented either by vector of **cell positions**(this is the representation of where cells would be on the game board), or with **block relative positioning**.

#### Block relative positioning:

Block relative positioning represents block in space, where block can be defined by one main cell, and other cells that are defined using main cell offset.
We use this positioning technique to be able to easily rotate the block.

How to rotate a block using block relative positioning?

Notice that: we can use main cell in block relative positioning, as the origin of 2d space.
Then, the offset cells, would simply be points in the vectors in that space.

Now, when we have linear space, we can apply rotation matrices to rotate the offset vectors, and therefore get new set of vectors that represent rotated block.
Also note that by doing this, our main cell never moves.

