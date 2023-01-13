
mod map_parser;
use raylib::prelude::*;

const RECT_WIDTH: i32 = 25;
const RECT_HEIGHT: i32 = 25;

fn main() {
	let map = map_parser::parse_map("ressources/map.txt".to_string());

	let (mut rl, thread) = raylib::init()
		.size(RECT_WIDTH * map.width, RECT_HEIGHT * map.height)
		.title("Hello, World")
		.build();
	 
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		 
		d.clear_background(Color::WHITE);
		
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		for &pos in map.map.iter() {
			if pos == 1
			{
				d.draw_rectangle(x * RECT_WIDTH, y * RECT_HEIGHT, RECT_WIDTH, RECT_HEIGHT, Color::RED);
			}
			else if pos == 2
			{
				d.draw_rectangle(x * RECT_WIDTH, y * RECT_HEIGHT, RECT_WIDTH, RECT_HEIGHT, Color::BLUE);
			}

			x = x + 1;
			if x == map.width
			{
				y = y + 1;
				x = 0;
			}
		}
	}
}