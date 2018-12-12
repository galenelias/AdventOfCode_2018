
fn power(x: i32, y: i32, grid_serial_num: i32) -> i32 {
	let rack_id = x + 10;
	let power_level = (rack_id * y + grid_serial_num) * rack_id;
	let partial = (power_level / 100) % 10;
	partial - 5
}

pub fn solve(inputs : Vec<String>) {
	let serial_num = inputs[0].parse::<i32>().unwrap();
	let mut grid = vec![vec![0; 300]; 300];

	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			grid[y][x] = power(x as i32, y as i32, serial_num);
		}
	}

	// let mut max = 0;
	for n in 1..=grid.len() {
		for y in 0..grid.len() - n {
			for x in 0..grid[y].len() - n {
				let mut total = 0;
				for dy in 0..n {
					for dx in 0..n {
						total += grid[y + dy][x + dx];
					}
				}

				if total == 121 { //}>= max {
					println!("{},{},{} = {}", x, y, n, total);
					return;
					// max = total;
				}
			}
		}
	}
}