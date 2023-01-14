#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs, fmt};
use std::str::FromStr;
use crate::raycasting::get_incr_for_angle;
pub struct Map 
{
	pub width: i32,
	pub height: i32,
	pub map: Vec<u8>,
	pub player: Player
}

pub struct Player
{
	pub x: f32,
	pub y: f32,
	pub fov: i32,
	pub angle: f32
}

impl fmt::Display for Player 
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "x : {}\ny : {}\nfov : {}\nangle : {}", self.x, self.y, self.fov, self.angle)
	}
}

pub fn parse_map(filename: String) -> Map
{
	let map_file = fs::read_to_string(filename).expect("Should read map");

	let lines: Vec<&str> = map_file.lines().collect();

	let first_line: Vec<i32> =  lines.get(0).unwrap()
											.split(",")
											.into_iter()
											.map(|x| i32::from_str(x).unwrap())
											.collect();

	let mut map_content: Vec<u8> = Vec::new();

	for line in lines.iter().skip(1) {
		map_content.append(&mut line.chars().map(|x| x as u8 - '0' as u8).collect());
	}

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
	
	
	let player_pos_index = map.map.iter().position(|&x| x == 2).unwrap();
	map.map[player_pos_index] = 0;
	let pos = transform_1d_to_2d(&map, player_pos_index as i32);

	map.player.x = pos.0 as f32 + 0.5;
	map.player.y = pos.1 as f32 + 0.5;
	map.player.fov = first_line[2];

	return map;
}

pub fn get_val_at_pos(map: &Map, li: i32, co: i32) -> u8
{
	// Check that we are not outside of the map
	assert!(li < map.height);
	assert!(co < map.width);

	let index = transform_2d_to_1d(&map, li, co);

	return map.map[index as usize];
}

fn transform_2d_to_1d(map: &Map, li: i32, co: i32) -> i32
{
	// Check that we are not outside of the map
	assert!(li < map.height);
	assert!(co < map.width);

	return li * map.width + co;
}

fn transform_1d_to_2d(map: &Map, index: i32) -> (i32, i32)
{
	// Check that we are not outside of the map
	assert!(index < map.map.len() as i32);

	let li = index / map.width;
	let co = index % map.width;

	return (co, li);
}

pub fn update_player_position(map: &mut Map, newx : f32, newy: f32)
{
	let new_1d_pos = transform_2d_to_1d(map, (map.player.y + newy) as i32, (map.player.x + newx) as i32);

	if map.map[new_1d_pos as usize] != 1 
	{
		map.player.x += newx;
		map.player.y += newy;
	}
}

pub fn update_player_angle(map: &mut Map, delta: f32)
{
	map.player.angle = (map.player.angle as f32 + delta) % 360.0;
}