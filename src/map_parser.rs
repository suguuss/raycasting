use std::{fs};
use std::str::FromStr;

pub struct Map 
{
	pub width: i32,
	pub height: i32,
	pub map: Vec<u8>
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

	

	let map: Map = Map {
		width: first_line[0],
		height: first_line[1],
		map: map_content
	};

	return map;
}

pub fn get_val_at_pos(map: Map, li: i32, co: i32) -> u8
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

	return (li, co);
}