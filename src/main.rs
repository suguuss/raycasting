
mod map;
use map::*;
// use raylib::RaylibHandle::is_key_down;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const RECT_WIDTH: i32 = 25;
const RECT_HEIGHT: i32 = 25;
const PLAYER_BASE_SPEED_DIVIDER: f32 = 200.0;

fn main() {
	let mut map = map::parse_map("ressources/map.txt".to_string());

	let (mut rl, thread) = raylib::init()
		.size(RECT_WIDTH * map.width, RECT_HEIGHT * map.height)
		.title("Raycasting 2D")
		.build();
	 
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		 
		// ! HANDLE INPUTS

		let mut speed_divider = PLAYER_BASE_SPEED_DIVIDER;

		if d.is_key_down(KEY_LEFT_SHIFT) {speed_divider = 100.0;}

		let speed: f32 = 0.5 / speed_divider;
		if d.is_key_down(KEY_A) {update_player_position(&mut map, -speed, 0.0)}
		if d.is_key_down(KEY_D) {update_player_position(&mut map, speed, 0.0)}
		if d.is_key_down(KEY_W) {update_player_position(&mut map, 0.0, -speed)}
		if d.is_key_down(KEY_S) {update_player_position(&mut map, 0.0, speed)}

		// ! DRAWING
		d.clear_background(Color::WHITE);
		
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		for &pos in map.map.iter() 
		{
			if pos == 1
			{
				d.draw_rectangle(x * RECT_WIDTH, y * RECT_HEIGHT, RECT_WIDTH, RECT_HEIGHT, Color::RED);
			}

			x = x + 1;
			if x == map.width
			{
				y = y + 1;
				x = 0;
			}
		}
		d.draw_circle((map.player.x * RECT_WIDTH as f32) as i32, (map.player.y * RECT_HEIGHT as f32) as i32, 10.0, Color::BLUE);
	}
}