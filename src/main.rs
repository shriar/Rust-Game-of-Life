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
    fade_grid: Vec<Vec<f32>>,
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
            fade_grid: vec![vec![0.0; cols as usize]; rows as usize],
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
                self.fade_grid[row as usize][col as usize] = 1.0;
            }
        }

        // Right click to erase
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let col = (mx / cell_w) as i32;
            let row = (my / cell_h) as i32;

            if col >= 0 && col < self.grid_cols && row >= 0 && row < self.grid_rows {
                self.cells[row as usize][col as usize] = false;
                // Don't clear fade immediately for a nice effect, or do? Let's let it decay.
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
                    self.fade_grid[row as usize][col as usize] = 0.0;
                }
            }
            self.running = false; // Also pause the simulation
            self.frame_count = 0; // Reset frame counter
        }

        // Speed control
        if is_key_pressed(KeyCode::Up) {
            self.update_delay = (self.update_delay - 1).max(1);
        }
        if is_key_pressed(KeyCode::Down) {
            self.update_delay = (self.update_delay + 1).min(60);
        }
    }

    fn update(&mut self) {
        // Update fade grid every frame for smooth animation
        for row in 0..self.grid_rows {
            for col in 0..self.grid_cols {
                if self.cells[row as usize][col as usize] {
                    self.fade_grid[row as usize][col as usize] = 1.0;
                } else {
                    self.fade_grid[row as usize][col as usize] *= 0.90; // Decay
                }
            }
        }

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
                            if neighbors == 3 {
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

        // Modern Dark Theme
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Draw colored cells with a slight margin for a "grid" look without lines
        let margin = 1.0;
        let draw_w = (cell_w - margin).max(1.0);
        let draw_h = (cell_h - margin).max(1.0);

        for row in 0..self.grid_rows {
            for col in 0..self.grid_cols {
                let fade = self.fade_grid[row as usize][col as usize];
                if fade > 0.05 {
                    // Only draw if visible enough
                    let x = col as f32 * cell_w + margin / 2.0;
                    let y = row as f32 * cell_h + margin / 2.0;

                    // Neon Green/Cyan color with alpha based on fade
                    // Alive cells are full brightness, dying cells fade out
                    let color = if self.cells[row as usize][col as usize] {
                        Color::new(0.0, 1.0, 0.8, 1.0) // Alive color
                    } else {
                        Color::new(0.0, 0.8, 0.6, fade) // Trail color
                    };

                    draw_rectangle(x, y, draw_w, draw_h, color);
                }
            }
        }

        // Optional: Very faint grid lines if cells are large enough, otherwise skip
        if cell_w > 5.0 {
            for i in 0..=self.grid_cols {
                let x = i as f32 * cell_w;
                draw_line(
                    x,
                    0.0,
                    x,
                    screen_height(),
                    1.0,
                    Color::new(1.0, 1.0, 1.0, 0.05),
                );
            }
            for j in 0..=self.grid_rows {
                let y = j as f32 * cell_h;
                draw_line(
                    0.0,
                    y,
                    screen_width(),
                    y,
                    1.0,
                    Color::new(1.0, 1.0, 1.0, 0.05),
                );
            }
        }

        // HUD
        let status = if self.running { "RUNNING" } else { "PAUSED" };
        let fps = get_fps();

        // Draw a semi-transparent background for the HUD
        draw_rectangle(10.0, 10.0, 280.0, 110.0, Color::new(0.0, 0.0, 0.0, 0.7));

        draw_text(&format!("Status: {}", status), 20.0, 30.0, 20.0, WHITE);
        draw_text(&format!("FPS: {}", fps), 20.0, 50.0, 20.0, WHITE);
        draw_text(
            &format!("Speed (Delay): {}", self.update_delay),
            20.0,
            70.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Controls: Space (Toggle), C (Clear)",
            20.0,
            90.0,
            16.0,
            LIGHTGRAY,
        );
        draw_text(
            "Up/Down (Speed), Left/Right Click",
            20.0,
            110.0,
            16.0,
            LIGHTGRAY,
        );
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
