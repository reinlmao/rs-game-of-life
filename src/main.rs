#![windows_subsystem = "windows"]

use ::rand::prelude::*;
use macroquad::prelude::*;

struct Field {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { alive: false }
    }

    /// Update Cell living state, setting it to `alive`
    pub fn update(&mut self, alive: bool) {
        self.alive = alive;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "cells".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Un-comment for detailed tracing data at runtime
    // std::env::set_var("RUST_BACKTRACE", "full");

    let mut rng = thread_rng();

    // Initialize the Cell vector
    const SIZE: Vec2 = Vec2 { x: 100., y: 100. };
    let mut cell_vec = Vec::with_capacity(SIZE.x as usize);
    for _x in 0..(SIZE.x as i32) {
        cell_vec.push(Vec::with_capacity(SIZE.y as usize));
    }
    for x in 0..(SIZE.x as i32) {
        for _y in 0..(SIZE.y as i32) {
            let mut new_cell = Cell::new();
            new_cell.alive = rng.gen_bool(0.5);
            cell_vec[x as usize].push(new_cell);
        }
    }

    let mut field = Field { cells: cell_vec };

    // Game Loop
    loop {
        clear_background(BLACK);

        let frametime = std::time::SystemTime::now();

        // Iterate through cells, tracking position
        let mut x_pos = 0;
        let immut_cells = field.cells.to_vec(); // Immutable copy of `field.cells`
        for arr in &mut field.cells {
            let mut y_pos = 0;
            for cell in arr {
                // Count neighbors' living state
                let mut alive_count: i8 = 0;
                for x_n in -1..=1 as i32 {
                    for y_n in -1..=1 as i32 {
                        if (x_pos + x_n >= 0 && x_pos + x_n <= 99)
                            && (y_pos + y_n >= 0 && y_pos + y_n <= 99)
                            && (Vec2 {
                                x: (x_pos + x_n) as f32,
                                y: (y_pos + y_n) as f32,
                            } != Vec2 {
                                x: x_pos as f32,
                                y: y_pos as f32,
                            })
                            && immut_cells[(x_pos + x_n) as usize][(y_pos + y_n) as usize].alive
                        {
                            alive_count += 1;
                        }
                    }
                }

                // Apply 'Game of Life' rules
                if cell.alive {
                    if alive_count > 1 && alive_count < 4 {
                        cell.update(true);
                    } else {
                        cell.update(false);
                    }
                } else if alive_count == 3 {
                    cell.update(true);
                }

                // Check mouse distance to Cell and randomize alive state if near
                if is_mouse_button_down(MouseButton::Left) {
                    let real_pos = Vec2 {
                        x: (x_pos * 8 + 4) as f32,
                        y: (y_pos * 8 + 4) as f32,
                    };
                    let mouse_pos = Vec2 {
                        x: mouse_position().0,
                        y: mouse_position().1,
                    };

                    if real_pos.distance(mouse_pos) < 32. {
                        cell.update(rng.gen_bool(0.5));
                    }
                }

                // Render Cell if alive
                if cell.alive {
                    draw_rectangle(x_pos as f32 * 8., y_pos as f32 * 8., 8., 8., WHITE);
                }
                y_pos += 1;
            }
            x_pos += 1;
        }

        // Un-comment next lines to slow the simulation
        // let factor = 25.;
        // std::thread::sleep(std::time::Duration::new(1, 0).mul_f32(1. / factor));

        // Show frametime for optimization benchmark
        draw_text(
            format!(
                "FrameTime: {}ms",
                frametime.elapsed().unwrap().as_secs_f32() * 1000.
            )
            .as_str(),
            5.,
            20.,
            20.,
            GREEN,
        );

        next_frame().await;
    }
}
