use itertools::Itertools;
use std::collections::VecDeque;

// Fill from given row/column, returning how many squares were filled
fn fill(grid: &mut Vec<Vec<(usize, i32)>>, r: i32, c: i32) -> usize {
	let ch = grid[r as usize][c as usize].1;

	let mut queue = VecDeque::new();
	queue.push_back((r, c));

	let mut area = 0;
	while !queue.is_empty() {
		let c = queue.pop_front().unwrap();

		if c.0 < 0 || c.0 >= (grid.len() as i32) || c.1 < 0 || c.1 >= (grid[0].len() as i32) {
			continue;
		} else if grid[c.0 as usize][c.1 as usize].1 == -1 || grid[c.0 as usize][c.1 as usize].1 != ch {
			continue;
		}

		area += 1;
		grid[c.0 as usize][c.1 as usize].1 = -1;

		queue.push_back((c.0 - 1, c.1    ));
		queue.push_back((c.0 + 1, c.1    ));
		queue.push_back((c.0    , c.1 - 1));
		queue.push_back((c.0    , c.1 + 1));
	}
	return area;	
}

fn dist(r1: usize, r2: usize, c1: usize, c2: usize) -> usize {
	(((r1 as i32) - (r2 as i32)).abs() + ((c1 as i32) - (c2 as i32)).abs()) as usize
}

pub fn solve(inputs : Vec<String>) {
	let pts = inputs.iter().map(|line| line.split(", ").map(|w| w.parse::<usize>().unwrap()).collect_vec()).collect_vec();

	let max_dim = 1 + pts.iter().map(|pt| std::cmp::max(pt[0], pt[1])).max().unwrap();

	// Grid is a 2D vector of tuples, representing (distance from nearest point, index of nearest point)
	// Index of '-1' indicates a tie for nearest point
	let mut grid = vec![ vec![(1000, 0i32); max_dim]; max_dim];

	for (i, pt) in pts.iter().enumerate() {
		for r in 0..max_dim {
			for c in 0..max_dim {
				let dist = dist(pt[0], r, pt[1], c);
				if dist < grid[r][c].0 {
					grid[r][c].0 = dist;
					grid[r][c].1 = i as i32;
				} else if dist == grid[r][c].0 {
					grid[r][c].1 = -1;
				}
			}
		}
	}


	// Fill all the regions which touch the edge, so they can't win in the next part
	for x in 0..(max_dim as i32) {
		fill(&mut grid, 0, x as i32);
		fill(&mut grid, (max_dim - 1) as i32, x);
		fill(&mut grid, x, 0);
		fill(&mut grid, x, (max_dim - 1) as i32);
	}

	_print_grid(&grid);


	let part1 = (0..max_dim).cartesian_product(0..max_dim)
					.map(|(r, c)| fill(&mut grid, r as i32, c as i32))
					.max().unwrap();
	println!("Part 1: {}", part1);

	let part2 = (0..max_dim).cartesian_product(0..max_dim)
					.filter(|&(r, c)|
						pts.iter().map(|pt| dist(pt[0], r, pt[1], c)).sum::<usize>() < 10000
					).count();
	println!("Part 2: {}", part2);
}


// Fun for debugging!
fn _print_grid(grid: &Vec<Vec<(usize, i32)>>) {
	for row in grid {
		for c in row {
			if c.1 == -1 {
				print!(".");
			} else if c.1 < 26 {
				let ch = (('a' as u8) + (c.1 as u8)) as char;
				print!("{}", ch);
			} else {
				let ch = ('a' as u8 + (c.1 as u8) - 26) as char;
				print!("{}", ch);
			}
		}
		println!("");
	}
}
