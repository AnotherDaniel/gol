#![allow(dead_code)]

use std::cmp;
use rand::prelude::*;
use nannou::prelude::*;

const X: usize = 384;
const Y: usize = 256;
const R: f32 = 0.5;
const P: u64 = 1010000101;

struct Model {
    _window: window::Id,
    _cells: [[bool; Y]; X],
}

fn init_cells_rand(threshold: f32) -> [[bool; Y]; X] {
    let mut rng = rand::thread_rng();
    let mut cells = [[false; Y]; X];
    
    for (_, row) in cells.iter_mut().enumerate() {
        for (_, cell) in row.iter_mut().enumerate() {
            let y: f32 = rng.gen();
            if y < threshold {
                *cell = true;
            }  
        }
    }
    return cells
}

fn init_cells_bitpattern(pattern: u64) -> [[bool; Y]; X] {
    let mut cells = [[false; Y]; X];
    
    for (_, row) in cells.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if pattern >> j%64 > 0 {
                *cell = true;
            }
        }
    }
    return cells
}

fn init_cells_bitrandom() -> [[bool; Y]; X] {
    let mut cells = [[false; Y]; X];
    let pattern = rand::thread_rng().gen::<u32>();

    for (_, row) in cells.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if pattern >> j%64 > 0 {
                *cell = true;
            }
        }
    }
    return cells
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size((X*3) as u32, (Y*3) as u32).view(view).build().unwrap();    
    let mut _cells = init_cells_rand(R);
//    let mut _cells = init_cells_bitpattern(P);
//    let mut _cells = init_cells_bitrandom();

    Model { _window, _cells  }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    count_neighbours(&mut _model._cells);
}

fn count_neighbours(cells: &mut [[bool; Y]; X]) {
    let mut live;
    let mut cells_ng: [[bool; Y]; X] = [[false;Y];X];   // this is our next-generation matrix

    for i in 0..X {
        for j in 0..Y {
            live = 0;

            // get 3x3 area around current cell via array slices (take care of fringe cell indices), count live neighbours (plus current cell)
            let col_slice = &cells[
                    (cmp::max(0 as i32, i as i32 - 1) as usize)
                    ..
                    cmp::min(X, i+2)
                ];
            for col in col_slice {
                let row_slice = &col[
                    (cmp::max(0 as i32, j as i32 - 1) as usize)
                    ..
                    cmp::min(Y, j+2)
                ];
                for row in row_slice {
                    if *row { live += 1; }
                }
            }

            // apply Conway game of life rules
            if cells[i][j] {
                // 1. Any live cell with two or three live neighbours survives.
                //    NOTE: as we've been counting neighbours PLUS the current cell above, we compare to 3&4 instead of 2&3
                if live == 3 || live == 4 {
                    cells_ng[i][j] = true;
                    continue;
                } 
            } else {
                // 2. Any dead cell with three live neighbours becomes a live cell.
                if live == 3 {
                    cells_ng[i][j] = true;
                    continue;
                } 
            }
            // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
            cells_ng[i][j] = false;
        }
    }

    // after counting and assessing next-gen status, copy ng matrix over into current gen
    for i in 0..X {
        for j in 0..Y {
            cells[i][j] = cells_ng[i][j];
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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    
    draw.background().color(WHITE);
    draw_cells(&win, &draw, &_model._cells);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
