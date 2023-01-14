#![allow(dead_code)]
#![allow(unused_variables)]

mod map;
use map::*;

mod raycasting;
use raycasting::*;

use raylib::{consts::KeyboardKey::*, prelude::*};

const WIDTH : i32 = 720;
const HEIGHT: i32 = 480;

fn main() {
	let mut map = parse_map("ressources/map.txt".to_string());


	let (mut rl, thread) = raylib::init()
		// .size(RECT_WIDTH * map.width, RECT_HEIGHT * map.height)
		.size(WIDTH, HEIGHT)
		.title("Raycasting 3D")
		.build();
	
	while !rl.window_should_close() {
		#[allow(unused_mut)]
		let mut d = rl.begin_drawing(&thread);
		 
		// ! HANDLE INPUTS
		let mut speed_divider = PLAYER_BASE_SPEED_DIVIDER;

		if d.is_key_down(KEY_LEFT_SHIFT) {speed_divider /= 2.0;}

		let speed: f32 = 0.5 / speed_divider;
		if d.is_key_down(KEY_W) {update_player_position(&mut map, speed, 0.0)}
		if d.is_key_down(KEY_S) {update_player_position(&mut map, speed, -180.0)}
		if d.is_key_down(KEY_A) {update_player_position(&mut map, speed, -90.0)}
		if d.is_key_down(KEY_D) {update_player_position(&mut map, speed, 90.0)}
		if d.is_key_down(KEY_Q) {update_player_angle(&mut map, -20.0 / 150.0)}
		if d.is_key_down(KEY_E) {update_player_angle(&mut map, 20.0 / 150.0)}

		let rays = shoot_rays(&map);

		d.clear_background(Color::SKYBLUE);
		d.draw_rectangle(0, HEIGHT/2, WIDTH, HEIGHT/2, Color::BROWN);

		for ray in rays.iter().enumerate() 
		{
			let rect_size = (WIDTH as f32 / rays.len() as f32).ceil() as i32;

			let ray_nb = ray.0 as i32;
			let dist_x = (ray.1.0 - map.player.x).abs();
			let dist_y = (ray.1.1 - map.player.y).abs();

			let angle = ray_nb as f32 - map.player.fov as f32/2.0;
			let distortion = angle.to_radians().cos().abs();
			let total_dist = dist_x.hypot(dist_y) * distortion;

			let mut wall_size = (HEIGHT as f32 * 0.80) - total_dist * 16.0;

			if wall_size < 50.0 {wall_size = 50.0;}

			let wall_color = Color {
				r: (wall_size / (1.1 * HEIGHT as f32) * 255.0) as u8,
				g: (wall_size / (1.1 * HEIGHT as f32) * 255.0) as u8,
				b: (wall_size / (1.1 * HEIGHT as f32) * 255.0) as u8,
				a: 255,
			};

			d.draw_rectangle(ray_nb * rect_size, HEIGHT/2 - (wall_size/2.0) as i32, rect_size, wall_size as i32, wall_color);
		}

		draw_2d_map(&mut d, &map, &rays);
	}
}