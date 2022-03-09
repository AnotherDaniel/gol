#![allow(dead_code)]
//#![allow(unused_variables)]

use std::time::Duration;
use rand::prelude::*;
use nannou::prelude::*;

const X: usize = 375;
const Y: usize = 300;
const R: usize = 10;

struct Model {
    _window: window::Id,
    _cells: [[bool; Y]; X],
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();    
    let mut _cells = init_cells_rand();
    //app.set_loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(R as u64) } );

    Model { _window, _cells  }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    count_neighbours(&mut _model._cells);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    
    draw.background().color(WHITE);
    draw_cells(&win, &draw, &_model._cells);
    draw.to_frame(app, &frame).unwrap();
}

fn init_cells_mod() -> [[bool; Y]; X] {
    let mut cells = [[false; Y]; X];

    for (i, row) in cells.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if ( i%8 == 0) && (j%6 == 0) {
                *cell = true;
            }
            if ( i%7 == 0) && (j%5 == 0) {
                *cell = true;
            }
            if ( i%6 == 0) && (j%4 == 0) {
                *cell = true;
            }
            if ( i%5 == 0) && (j%3 == 0) {
                *cell = true;
            }
        }
    }
    return cells
}

fn init_cells_rand() -> [[bool; Y]; X] {
    let thresh = 0.022;
    let mut rng = rand::thread_rng();
    let mut cells = [[false; Y]; X];
    
    for (_, row) in cells.iter_mut().enumerate() {
        for (_, cell) in row.iter_mut().enumerate() {
            let y: f64 = rng.gen();
            if y > thresh {
                *cell = true;
            }  
        }
    }
    return cells
}

fn count_neighbours(cells: &mut [[bool; Y]; X]) {
    let mut live = 0;

    for i in 0..X {
        for j in 0..Y {
            if i > 0 && j > 0 {
                if cells[i-1][j-1] {
                    live += 1;
                }
            }
            if j > 0 {
               if cells[i][j-1] {
                    live += 1;
                }
            }
            if i < cells.len()-1 && j > 0 {
                if cells[i+1][j-1] {
                    live += 1;
                }
            }

            if i > 0 {
               if cells[i-1][j] {
                    live += 1;
                }
            }
            if i < cells.len()-1 {
               if cells[i+1][j] {
                    live += 1;
                }
            }

            if i > 0 && j < cells[i].len()-1 {
                if cells[i-1][j+1] {
                    live += 1;
                }
            }
            if j < cells[i].len()-1 {
                if cells[i][j+1] {
                    live += 1;
                }
            }
            if i < cells.len()-1 && j < cells[i].len()-1 {
                if cells[i+1][j+1] {
                    live += 1;
                }
            }

            if cells[i][j] {
                // 1. Any live cell with two or three live neighbours survives.
                // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                if live < 2 || live > 3 {
                    cells[i][j] = false;
                }
            } else {
                // 2. Any dead cell with three live neighbours becomes a live cell.
                if live == 3 {
                    cells[i][j] = true;
                }
            }
            live = 0;
        }
    }
}

fn draw_cells(win: &Rect, draw: &Draw, cells: &[[bool; Y]; X]) {
    let win_p = win.pad(10.0);
    let r = Rect::from_w_h(win_p.w(), win_p.h()).top_left_of(win_p);
    let x_step = r.w() / X as f32;
    let y_step = r.h() / Y as f32;
    let mut live_color;

    for (i, row) in cells.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if cells[i][j] {
                live_color = gray(0.1);
            } else {
                live_color = gray(0.9);
            }

            draw.rect().color(live_color)
                .x_y(r.left() + ((i as f32) * x_step), r.top() - ((j as f32)*y_step))
                .w_h(x_step, y_step);
        }
    }
}
