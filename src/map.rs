#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::str::FromStr;

use macroquad::prelude::*;

use crate::raycasting::get_incr_for_angle;

#[derive(Debug)]
pub struct Map 
{
	pub width: i32,
	pub height: i32,
	pub map: Vec<u8>,
	pub player: Player
}

#[derive(Debug)]
pub struct Player
{
	pub x: f32,
	pub y: f32,
	pub fov: i32,
	pub angle: f32
}

// Read the mpa file and creates the map and player
pub fn parse_map(filename: String) -> Map
{
	// Read the map from the file
	// let map_file = fs::read_to_string(filename).expect("Should read map");
	let map_file = "16,16,50
1111111111111111
1111111111100001
1000010010100001
1000010010100001
1000000010100001
1000000000000001
1000020000000001
1000000000000001
1000000000000001
1000000011000001
1000001111000001
1000001100000001
1000000000000001
1000000000000001
1000000000000001
1111111111111111";

	println!("{}", map_file);
	let lines: Vec<&str> = map_file.lines().collect();

	// Get the values on the first line of the file
	// WIDTH, HEIGHT, PLAYER_FOV
	let first_line: Vec<i32> =  lines.get(0).unwrap()
											.split(",")
											.into_iter()
											.map(|x| i32::from_str(x).unwrap())
											.collect();

	// Fill the map inside a 1d array
	let mut map_content: Vec<u8> = Vec::new();

	for line in lines.iter().skip(1) {
		map_content.append(&mut line.chars().map(|x| x as u8 - '0' as u8).collect());
	}

	// Create the map and the player
	let player = Player {
		x: 0.0,
		y: 0.0,
		fov: 0,
		angle: 0.0
	};

	let mut map: Map = Map {
		width: first_line[0],
		height: first_line[1],
		map: map_content,
		player: player
	};
	
	// Search the initial position of the player in the map
	let player_pos_index = map.map.iter().position(|&x| x == 2).expect("Player should be placed in map");
	map.map[player_pos_index] = 0;
	let pos = transform_1d_to_2d(&map, player_pos_index as i32);

	// Place the player at the correct coordinates
	map.player.x = pos.0 as f32 + 0.5;
	map.player.y = pos.1 as f32 + 0.5;
	map.player.fov = first_line[2];

	return map;
}

// Get a value at the pos li co in the map
pub fn get_val_at_pos(map: &Map, li: i32, co: i32) -> u8
{
	// Check that we are not outside of the map
	assert!(li < map.height);
	assert!(co < map.width);

	let index = transform_2d_to_1d(&map, li, co);

	return map.map[index as usize];
}

// Transform 2d coordinates to 1d coordinates
fn transform_2d_to_1d(map: &Map, li: i32, co: i32) -> i32
{
	// Check that we are not outside of the map
	assert!(li < map.height);
	assert!(co < map.width);

	return li * map.width + co;
}

// Transform 1d coordinates to 2d coordinates
fn transform_1d_to_2d(map: &Map, index: i32) -> (i32, i32)
{
	// Check that we are not outside of the map
	assert!(index < map.map.len() as i32);

	let li = index / map.width;
	let co = index % map.width;

	return (co, li);
}

// Update the player's position
pub fn update_player_position(map: &mut Map, speed: f32, angle_delta: f32)
{
	// Get the x and y incrementation values depending on the angle
	let incr = get_incr_for_angle(map.player.angle + angle_delta);

	let newx = incr.0 * speed;
	let newy = incr.1 * speed;

	// Check walls collisions
	let new_1d_pos = transform_2d_to_1d(map, (map.player.y + newy) as i32, (map.player.x + newx) as i32);

	if map.map[new_1d_pos as usize] != 1 
	{
		map.player.x += newx;
		map.player.y += newy;
	}
}

// Update the angle of the player
pub fn update_player_angle(map: &mut Map, delta: f32)
{
	map.player.angle = (map.player.angle as f32 + delta) % 360.0;
}

// ! DRAWING

pub fn draw_2d_map(map: &Map, rays: &Vec<(f32, f32)>)
{
	let rect_width : f32 = screen_width() / 6.0 / map.width as f32;
	let rect_height: f32 = screen_width() / 6.0 / map.height as f32;

	let player_x_pxl_pos = (map.player.x * rect_width) as i32;
	let player_y_pxl_pos = (map.player.y * rect_height) as i32;

	for ray in rays
	{
		draw_line(
			map.player.x * rect_width, 
			map.player.y * rect_height, 
			ray.0 * rect_width, 
			ray.1 * rect_height, 
			1.0,
			GREEN
		);
	}

	let mut x: f32 = 0.0;
	let mut y: f32 = 0.0;
	for &pos in map.map.iter() 
	{
		if pos == 1 {
			draw_rectangle(
				x * rect_width,
				y * rect_height,
				rect_width,
				rect_height,
				RED
			);
		}


		x = x + 1.0;
		if x == map.width as f32
		{
			y = y + 1.0;
			x = 0.0;
		}
	}
	draw_circle(map.player.x * rect_width, map.player.y * rect_height, 3.0, BLUE);
}