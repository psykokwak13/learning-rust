use macroquad::prelude::*;
use ::rand::{RngExt, rng};

const CELL_SIZE : f32 = 4.0;
const WIDTH : f32 = 800.0;
const HEIGHT : f32 = 600.0;

fn init_grid() -> Vec<Vec<bool>> {
	let mut result : Vec<Vec<bool>> = Vec::new();

	for _i in (10..HEIGHT as i32).step_by(10) {
		let mut row : Vec<bool> = Vec::new();
		for _j in (10..WIDTH as i32).step_by(10) {
			row.push(rng().random_bool(0.4));
		}
		result.push(row);
	}

	result
}

// count neighbors around the cell
fn count_neighbor_alive(grid : &Vec<Vec<bool>>, i : i32, j : i32) -> i8 {
	let mut count = 0;
	let width : i32 = (*grid)[i as usize].len() as i32;
	let height : i32 = (*grid).len() as i32;

	// loop for checking only neighbors of the cell
	for y in -1..=1i32 {
		let y_check = y + i;
		for x in -1..=1i32 {
			let x_check = x + j;
			if x == 0 && y == 0 { continue ; }
			if (y_check >= 0 && y_check < height) && (x_check >= 0 && x_check < width) {
				if (*grid)[y_check as usize][x_check as usize] {
					count += 1;
				}
			}
		}
	}
	count
}

// update every cell state to the next generation
fn game_of_life( grid : &mut Vec<Vec<bool>> ) {
	let mut clone_grid = grid.clone();

	for (i, h) in grid.iter().enumerate() {
		for j in 0..h.len() - 1 {
			let count = count_neighbor_alive(&grid, i as i32, j as i32 );
			clone_grid[i][j] = match (grid[i][j], count) {
				(true, 2) | (true, 3)	=> true,
				(false, 3)				=> true,
				_ 						=> false
			};
		}
}
	*grid = clone_grid;
}

// draw the cell in green if alive
fn draw_grid(grid : &Vec<Vec<bool>>) {
	let mut x = 0;
	let mut y = 0;

	for h in grid.iter() {
		for w in h.iter() {
			if *w {
				draw_rectangle(x as f32, y as f32, CELL_SIZE, CELL_SIZE, GREEN);
			}
			else {
				draw_rectangle(x as f32, y as f32, CELL_SIZE, CELL_SIZE, BLACK);
			}
			x += 10;
		}
		x = 0;
		y += 10;
	}
}

#[macroquad::main("game of life")]
async fn main() {
	// set screen size using constants defined above
	request_new_screen_size(WIDTH, HEIGHT);

	let mut grid = init_grid();

// main game loop
	loop {
		// wait until the next frame
		next_frame().await;

		clear_background(BLACK);
		game_of_life(&mut grid);
		draw_grid(&grid);
	}
}
