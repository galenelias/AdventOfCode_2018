use std::collections::HashMap;
use itertools::Itertools;

fn total(y: usize, x: usize, ch: char, grid: &Vec<Vec<char>>) -> usize {
	let y = y as i32;
	let x = x as i32;
	let mut result = 0;
	for dy in -1..=1 {
		for dx in -1..=1 {
			if y + dy < 0 || x + dx < 0 || (dy == 0 && dx == 0) || y + dy >= grid.len() as i32 || x + dx >= grid[0].len() as i32 {
				continue;
			}

			if grid[(y + dy) as usize][(x + dx) as usize] == ch {
				result += 1;
			}
		}
	}
	return result;
}

fn resource_value(grid: &str) -> usize {
	let wood = grid.chars().filter(|&ch| ch == '|').count();
	let mill = grid.chars().filter(|&ch| ch == '#').count();
	wood * mill
}

pub fn solve(inputs : Vec<String>) {
	let mut grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	// Store 'grid string' -> 'iteration #' mapping
	let mut memory : HashMap<String, u64> = HashMap::new();

	for i in 0u64.. {
		let prev_grid = grid.clone();
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				let ch = prev_grid[y][x];
				if ch == '.' && total(y, x, '|', &prev_grid) >= 3 {
					grid[y][x] = '|';
				} else if ch == '|' && total(y, x, '#', &prev_grid) >= 3 {
					grid[y][x] = '#';
				} else if ch == '#' && !(total(y, x, '#', &prev_grid) >= 1 && total(y, x, '|', &prev_grid) >= 1) {
					grid[y][x] = '.';
				}
			} 
		}

		let digest = grid.iter().map(|row| row.iter().collect::<String>()).join("");
		if i == 9 {
			println!("Part 1: {}", resource_value(&digest));
		}

		if memory.contains_key(&digest) {
			let prev_seen_iteration = memory.get(&digest).unwrap();
			let period = (i + 1) - prev_seen_iteration;
			let rem = (1000000000 - prev_seen_iteration) % period;

			for (grid_str, iter) in &memory {
				if (iter - prev_seen_iteration) == rem {
					println!("Part 2: {}", resource_value(grid_str));
					break;
				}
			}
			break;
		} else {
			memory.insert(digest, i+1);
		}
	}
}