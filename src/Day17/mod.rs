use regex::Regex;

fn drip(y_start: usize, x_start: usize, grid: &mut Vec<Vec<char>>) -> bool {
	for y in y_start..grid.len() {
		// Flow through existing wet sand
		if grid[y][x_start] == '|' || grid[y][x_start] == '.' {
			grid[y][x_start] = '|';
			continue;
		}

		// Hit something, start flowing. Check if we hit walls, then fill with '~', else, spread '|' and recurse
		let flow_y = y - 1;
		let mut made_progress = false;
		let mut hit_left_wall = false;
		let mut hit_right_wall = false;

		for x in (0..x_start).rev() {
			if grid[flow_y][x] == '#' {
				hit_left_wall = true;
				break;
			}
			grid[flow_y][x] = '|';
			if grid[y][x] != '#' && grid[y][x] != '~' {
				if drip(flow_y, x, grid) {
					made_progress = true;
				}
				break;
			}
		}

		for x in x_start..2000 {
			if grid[flow_y][x] == '#' {
				hit_right_wall = true;
				break;
			}
			grid[flow_y][x] = '|';
			if grid[y][x] != '#' && grid[y][x] != '~' {
				if drip(flow_y, x, grid) {
					made_progress = true;
				}
				break;
			}
		}

		// If we hit walls, then switch our '|'s to '~'s
		if hit_left_wall && hit_right_wall {
			made_progress = true;
			for x in (0..x_start).rev() {
				if grid[flow_y][x] != '|' {
					break;
				}
				grid[flow_y][x] = '~';
			}
			for x in x_start..grid[flow_y].len() {
				if grid[flow_y][x] != '|' {
					break;
				}
				grid[flow_y][x] = '~';
			}
		}
		return made_progress;
	}
	return false;
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"(\w)=(\d+), (\w)=(\d+)..(\d+)").unwrap();
	let debug_draw = false;

	let mut grid = vec![vec!['.'; 2000]; 2000];
	let mut min_y = usize::max_value();
	let mut max_y = 0;
	let mut min_x = usize::max_value();
	let mut max_x = 0;

	for input in inputs {
		let caps = re_input.captures(&input).unwrap();
		let c1 = caps[1].to_string();
		let c2 = caps[2].parse::<usize>().unwrap();
		let c4 = caps[4].parse::<usize>().unwrap();
		let c5 = caps[5].parse::<usize>().unwrap();

		if c1 == "x" {
			max_x = std::cmp::max(max_y, c2);
			min_x = std::cmp::min(min_y, c2);
			for y in c4..=c5 {
				max_y = std::cmp::max(max_y, y);
				min_y = std::cmp::min(min_y, y);
				grid[y][c2] = '#';
			}
		} else {
			min_y = std::cmp::min(min_y, c2);
			max_y = std::cmp::max(max_y, c2);
			for x in c4..=c5 {
				grid[c2][x] = '#';
				max_x = std::cmp::max(max_y, x);
				min_x = std::cmp::min(min_y, x);
			}
		}
	}

	let mut iterations = 0;
	loop {
		iterations += 1;
		if !drip(0, 500, &mut grid) {
			break;
		}

		if debug_draw {
			for r in min_y..=max_y {
				for c in min_x..=max_x {
					print!("{}", grid[r][c]);
				}
				println!("");
			}
			println!("");
		}
	}

	if debug_draw {
		for r in min_y..=max_y {
			for c in min_x..=max_x {
				print!("{}", grid[r][c]);
			}
			println!("");
		}
		println!("");
	}

	let part1 = grid.iter().skip(min_y).take(max_y-min_y+1).map(|row| row.iter().filter(|&c| c == &'~' || c == &'|').count()).sum::<usize>();
	println!("Stopped after {} iterations", iterations);
	println!("Part 1: Total water = {}", part1);

	let part2 = grid.iter().skip(min_y).take(max_y-min_y+1).map(|row| row.iter().filter(|&c| c == &'~').count()).sum::<usize>();
	println!("Part 2: Total settled water = {}", part2);
}