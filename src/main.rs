#![allow(dead_code)]
#![allow(unused_variables)]

mod map;
use map::*;

mod raycasting;
use raycasting::*;

use raylib::consts::KeyboardKey::*;


fn main() {
	let mut map = parse_map("ressources/map.txt".to_string());


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
		if d.is_key_down(KEY_W) {update_player_position(&mut map, speed, 0.0)}
		if d.is_key_down(KEY_S) {update_player_position(&mut map, speed, -180.0)}
		if d.is_key_down(KEY_A) {update_player_position(&mut map, speed, -90.0)}
		if d.is_key_down(KEY_D) {update_player_position(&mut map, speed, 90.0)}
		if d.is_key_down(KEY_Q) {update_player_angle(&mut map, -20.0 / 500.0)}
		if d.is_key_down(KEY_E) {update_player_angle(&mut map, 20.0 / 500.0)}

		let rays = shoot_rays(&map);

		draw_2d_map(&mut d, &map, &rays);
	}
}