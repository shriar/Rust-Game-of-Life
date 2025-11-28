use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "MyGame".to_string(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

struct GameState {
    cells: Vec<Vec<bool>>,
    running: bool,
    frame_count: i32,
    grid_rows: i32,
    grid_cols: i32,
    update_delay: i32,
}

impl GameState {
    fn new(rows: i32, cols: i32) -> Self {
        Self {
            cells: vec![vec![false; cols as usize]; rows as usize],
            running: false,
            frame_count: 0,
            grid_rows: rows,
            grid_cols: cols,
            update_delay: 1,
        }
    }

    fn calculate_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_row = row + i;
                let new_col = col + j;
                if new_row >= 0
                    && new_row < self.grid_rows
                    && new_col >= 0
                    && new_col < self.grid_cols
                {
                    if self.cells[new_row as usize][new_col as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn handle_input(&mut self) {
        let cell_w = screen_width() / self.grid_cols as f32;
        let cell_h = screen_height() / self.grid_rows as f32;

        // Handle mouse input to toggle cells
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let col = (mx / cell_w) as i32;
            let row = (my / cell_h) as i32;
            if col >= 0 && col < self.grid_cols && row >= 0 && row < self.grid_rows {
                self.cells[row as usize][col as usize] = true;
            }
        }

        // Right click to erase
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let col = (mx / cell_w) as i32;
            let row = (my / cell_h) as i32;

            if col >= 0 && col < self.grid_cols && row >= 0 && row < self.grid_rows {
                self.cells[row as usize][col as usize] = false;
            }
        }

        // Toggle simulation with spacebar
        if is_key_pressed(KeyCode::Space) {
            self.running = !self.running;
        }

        // Clear all cells with 'C' key
        if is_key_pressed(KeyCode::C) {
            for row in 0..self.grid_rows {
                for col in 0..self.grid_cols {
                    self.cells[row as usize][col as usize] = false;
                }
            }
            self.running = false; // Also pause the simulation
            self.frame_count = 0; // Reset frame counter
        }
    }

    fn update(&mut self) {
        if self.running {
            self.frame_count += 1;

            // Only update every UPDATE_DELAY frames
            if self.frame_count >= self.update_delay {
                self.frame_count = 0;

                let mut new_cells = self.cells.clone();
                for row in 0..self.grid_rows {
                    for col in 0..self.grid_cols {
                        let neighbors = self.calculate_neighbors(row, col);
                        if self.cells[row as usize][col as usize] {
                            if neighbors < 2 || neighbors > 3 {
                                new_cells[row as usize][col as usize] = false;
                            }
                        } else {
                            if neighbors == 3{
                                new_cells[row as usize][col as usize] = true;
                            }
                        }
                    }
                }
                self.cells = new_cells;
            }
        }
    }

    fn draw(&self) {
        let cell_w = screen_width() / self.grid_cols as f32;
        let cell_h = screen_height() / self.grid_rows as f32;

        clear_background(BLACK);

        // Draw colored cells
        for row in 0..self.grid_rows {
            for col in 0..self.grid_cols {
                if self.cells[row as usize][col as usize] {
                    let x = col as f32 * cell_w;
                    let y = row as f32 * cell_h;
                    draw_rectangle(x, y, cell_w, cell_h, WHITE);
                }
            }
        }

        // Draw grid lines on top
        for i in 0..=self.grid_cols {
            let x = i as f32 * cell_w;
            draw_line(x, 0.0, x, screen_height(), 1.0, DARKGRAY);
        }
        for j in 0..=self.grid_rows {
            let y = j as f32 * cell_h;
            draw_line(0.0, y, screen_width(), y, 1.0, DARKGRAY);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new(150, 150);

    loop {
        game_state.handle_input();
        game_state.update();
        game_state.draw();

        next_frame().await;
    }
}
