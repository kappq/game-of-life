use macroquad::prelude::*;

const WIDTH: usize = 128;
const HEIGHT: usize = 72;

const CELL_SIZE: f32 = 20.0;

const TARGET_FPS: f64 = 10.0;

struct Grid {
    cells: [[bool; WIDTH]; HEIGHT],
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: [[false; WIDTH]; HEIGHT],
        }
    }

    fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let i = (mouse_y / CELL_SIZE) as usize;
            let j = (mouse_x / CELL_SIZE) as usize;
            if i < HEIGHT && j < WIDTH {
                self.cells[i][j] = true;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();
            let i = (mouse_y / CELL_SIZE) as usize;
            let j = (mouse_x / CELL_SIZE) as usize;
            if i < HEIGHT && j < WIDTH {
                self.cells[i][j] = false;
            }
        }

        if is_key_pressed(KeyCode::C) {
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    self.cells[i][j] = false;
                }
            }
        }
    }

    fn update(&mut self) {
        let mut new_cells = self.cells;

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let live_neighbors = self.get_live_neighbors(i, j);

                if self.cells[i][j] && (live_neighbors < 2 || live_neighbors > 3) {
                    new_cells[i][j] = false;
                } else if !self.cells[i][j] && live_neighbors == 3 {
                    new_cells[i][j] = true;
                }
            }
        }

        self.cells = new_cells;
    }

    fn draw(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let x = j as f32 * CELL_SIZE;
                let y = i as f32 * CELL_SIZE;

                if self.cells[i][j] {
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLACK);
                } else {
                    draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 1.0, BLACK);
                }
            }
        }
    }

    fn get_live_neighbors(&self, i: usize, j: usize) -> usize {
        let mut live_neighbors = 0;

        let offsets = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (offset_i, offset_j) in offsets {
            let neighbor_i = i as i32 + offset_i;
            let neighbor_j = j as i32 + offset_j;
            if 0 <= neighbor_i
                && neighbor_i < HEIGHT as i32
                && 0 <= neighbor_j
                && neighbor_j < WIDTH as i32
                && self.cells[neighbor_i as usize][neighbor_j as usize]
            {
                live_neighbors += 1;
            }
        }

        live_neighbors
    }
}

#[macroquad::main(macroquad_conf)]
async fn main() {
    let mut grid = Grid::new();
    let mut running = false;

    let mut last_update_time = get_time();

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Enter) {
            running = true;
        }

        if !running {
            grid.handle_input();
        } else {
            let current_time = get_time();
            if current_time - last_update_time >= 1.0 / TARGET_FPS {
                grid.update();
                last_update_time = current_time;
            }
        }

        grid.draw();

        next_frame().await;
    }
}

fn macroquad_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: (WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (HEIGHT as f32 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
