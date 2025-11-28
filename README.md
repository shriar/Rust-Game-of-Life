# Rust Game of Life

A high-performance, visually enhanced implementation of Conway's Game of Life in Rust using the `macroquad` game engine.


## How to Run

Ensure you have Rust and Cargo installed. Then run:

```bash
cargo run
```

## Controls

| Key / Action | Function |
|--------------|----------|
| **Left Click** | Draw live cells |
| **Right Click** | Erase cells |
| **Space** | Toggle Play/Pause |
| **Up Arrow** | Increase Simulation Speed |
| **Down Arrow** | Decrease Simulation Speed |
| **C** | Clear the entire grid |

## The Rules of Life

1. **Underpopulation**: Any live cell with fewer than two live neighbours dies.
2. **Survival**: Any live cell with two or three live neighbours lives on to the next generation.
3. **Overpopulation**: Any live cell with more than three live neighbours dies.
4. **Reproduction**: Any dead cell with exactly three live neighbours becomes a live cell.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://github.com/not-fl3/macroquad)