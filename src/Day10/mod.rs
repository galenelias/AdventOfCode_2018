use itertools::Itertools;
use regex::Regex;

struct Particle {
	x: i32,
	y: i32,
	dx: i32,
	dy: i32,
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();

	let mut particles = inputs.iter().map(|line| {
		let caps = re_input.captures(&line).unwrap();
		Particle {
			x: caps[1].parse::<i32>().unwrap(),
			y: caps[2].parse::<i32>().unwrap(),
			dx: caps[3].parse::<i32>().unwrap(),
			dy: caps[4].parse::<i32>().unwrap(),
		}
	}).collect_vec();

	let mut have_found_in_range = false;
	for seconds in 0.. {
		let min_y = particles.iter().map(|p| p.y).min().unwrap();
		let max_y = particles.iter().map(|p| p.y).max().unwrap();
		let min_x = particles.iter().map(|p| p.x).min().unwrap();
		let max_x = particles.iter().map(|p| p.x).max().unwrap();

		if max_y - min_y < 100 && max_x - min_x < 100 {
			println!("Time: {}", seconds);
			println!("{} - {} ({}),   {} - {} ({})", min_y, max_y, max_y - min_y, min_x, max_x, max_x - min_x);
			for r in min_y..=max_y {
				for c in min_x..=max_x {
					if particles.iter().any(|p| p.x == c && p.y == r) {
						print!("#");
					} else {
						print!(".");
					}
				}
				println!("");
			}
			println!("");
			println!("");
			have_found_in_range = true;
		} else if have_found_in_range {
			break;
		}
		
		for particle in &mut particles {
			particle.x += particle.dx;
			particle.y += particle.dy;
		}
	}
}