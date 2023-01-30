#![allow(dead_code)]
#![allow(unused_variables)]

mod map;
use map::*;

mod raycasting;
use raycasting::*;

// use raylib::{consts::KeyboardKey::*, prelude::*};
use macroquad::prelude::*;

const WIDTH : i32 = 1260;
const HEIGHT: i32 = 768;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Raycasting 3D"),
        window_width: WIDTH,
        window_height: HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
	let mut map = parse_map("ressources/map.txt".to_string());
	
	loop {

		// ! HANDLE INPUTS
		let speed: f32 = 0.1;

		if is_key_down(KeyCode::LeftShift) {let speed: f32 = 0.15;}

		if is_key_down(KeyCode::W) {update_player_position(&mut map, speed, 0.0)}
		if is_key_down(KeyCode::S) {update_player_position(&mut map, speed, -180.0)}
		if is_key_down(KeyCode::A) {update_player_position(&mut map, speed, -90.0)}
		if is_key_down(KeyCode::D) {update_player_position(&mut map, speed, 90.0)}
		if is_key_down(KeyCode::Q) {update_player_angle(&mut map, -1.0)}
		if is_key_down(KeyCode::E) {update_player_angle(&mut map, 1.0)}

		let walls = shoot_rays(&map);

		clear_background(SKYBLUE);
		draw_rectangle(0.0, screen_height()/2.0, screen_width(), screen_height()/2.0, BROWN);
		
		let rect_size = (screen_width() / walls.len() as f32).ceil();

		for wall in walls.iter().enumerate() 
		{
			let wall_nb = wall.0 as f32;
			let dist_x = (wall.1.0 - map.player.x).abs();
			let dist_y = (wall.1.1 - map.player.y).abs();

			// Get the distance between the player and the wall and fix the distortion
			let angle = wall_nb as f32 / ANGLE_RESOLUTION as f32 - map.player.fov as f32/2.0;
			let distortion = angle.to_radians().cos().abs();
			let total_dist = dist_x.hypot(dist_y) * distortion;

			let mut wall_size = (screen_height() * 0.80) - total_dist * 16.0;

			if wall_size < 50.0 {wall_size = 50.0;}

			// Divide the distance by the max distance to get a value between 0 and 1
			let wall_color = Color {
				r: wall_size / (screen_height() * 0.8),
				g: wall_size / (screen_height() * 0.8),
				b: wall_size / (screen_height() * 0.8),
				a: 255.0,
			};

			draw_rectangle(wall_nb * rect_size, screen_height()/2.0 - (wall_size/2.0), rect_size as f32, wall_size, wall_color);
		}
		
		draw_2d_map(&map, &walls);

		next_frame().await
	}
}