fn power(x: i32, y: i32, grid_serial_num: i32) -> i32 {
	let rack_id = x + 10;
	let power_level = (rack_id * y + grid_serial_num) * rack_id;
	let partial = (power_level / 100) % 10;
	partial - 5
}

fn calc_max(grid: &Vec<Vec<i32>>, square_min: usize, square_max: usize) -> (i32, usize, usize, usize) {
	let mut max_params = (0, 0, 0, 0); // (sum, x, y, box_size)
	for n in square_min..=square_max {
		for y in 1..grid.len() - n {
			for x in 1..grid[y].len() - n {
				let sum = grid[y+n-1][x+n-1] + grid[y-1][x-1] - grid[y+n-1][x-1] - grid[y-1][x+n-1];
				if sum > max_params.0 {
					max_params = (sum, x, y, n);
				}
			}
		}
	}
	max_params
}

pub fn solve(inputs : Vec<String>) {
	let serial_num = inputs[0].parse::<i32>().unwrap();
	let mut grid = vec![vec![0; 301]; 301];

	for y in 1..grid.len() {
		for x in 1..grid[y].len() {
			grid[y][x] = power(x as i32, y as i32, serial_num);
		}
	}

	// Compute partial sums
	for y in 1..grid.len() {
		for x in 1..grid[y].len() {
			grid[y][x] = grid[y][x] + grid[y-1][x] + grid[y][x-1] - grid[y-1][x-1];
		}
	}

	let part1 = calc_max(&grid, 3, 3);
	println!("Part 1: {},{},{} = {}", part1.1, part1.2, part1.3, part1.0);

	let part2 = calc_max(&grid, 1, grid.len());
	println!("Part 2: {},{},{} = {}", part2.1, part2.2, part2.3, part2.0);
}