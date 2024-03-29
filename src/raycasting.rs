use crate::map::*;

pub const ANGLE_RESOLUTION: i32 = 3;

pub fn shoot_rays(map: &Map) -> Vec<(f32, f32)>
{
	let half_angle = map.player.fov as f32 / 2.0;
	let base_angle = map.player.angle;

	let mut walls: Vec<(f32, f32)> = Vec::new();

	for angle in (base_angle-half_angle) as i32 * ANGLE_RESOLUTION..(base_angle+half_angle) as i32 * ANGLE_RESOLUTION {
		let angle = f64::from(angle) / ANGLE_RESOLUTION as f64;
		let incr = get_incr_for_angle(angle as f32);
		
		let mut x = map.player.x;
		let mut y = map.player.y;

		while get_val_at_pos(&map, y as i32, x as i32) != 1 {
			x += incr.0 / 64.0;
			y += incr.1 / 64.0;
		}

		walls.append(&mut vec![(x, y)]);
	}

	return walls;
}

/**
 * Returns the x, y increment for a given angle
 */
pub fn get_incr_for_angle(angle: f32) -> (f32, f32)
{
	let incr_x = angle.to_radians().cos();
	let incr_y = angle.to_radians().sin();

	return (incr_x, incr_y);
}