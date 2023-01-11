use std::{fs};
use std::str::FromStr;

pub struct Map 
{
	width: u16,
	height: u16,
	map: Vec<u8>
}

pub fn parse_map(filename: String) -> Map
{
	let map_file = fs::read_to_string(filename).expect("Should read map");

	let lines: Vec<&str> = map_file.lines().collect();

	let first_line: Vec<u16> =  lines.get(0).unwrap()
												   .split(",")
												   .into_iter()
												   .map(|x| u16::from_str(x).unwrap())
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