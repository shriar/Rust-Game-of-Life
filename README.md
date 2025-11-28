# Rust Game of Life

A simple implementation of Conway's Game of Life in Rust using `macroquad`.

## How to Run

```bash
cargo run
```

## Controls

- **Left Click**: Draw cells
- **Right Click**: Erase cells
- **Space**: Pause/Resume simulation
- **C**: Clear grid

## Rules

1. Any live cell with fewer than two live neighbours dies (underpopulation).
2. Any live cell with two or three live neighbours lives.
3. Any live cell with more than three live neighbours dies (overpopulation).
4. Any dead cell with exactly three live neighbours becomes a live cell (reproduction).