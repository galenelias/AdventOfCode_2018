use std::collections::HashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
	Left,Up,Down,Right
}

#[derive(PartialEq, Eq, Debug)]
struct Cart {
	r: usize,
	c: usize,
	direction: Direction,
	turn_preference: usize, // left, straight, right	
}

fn turn_left(dir: &Direction) -> Direction {
	match dir {
		Direction::Up => Direction::Left,
		Direction::Right => Direction::Up,
		Direction::Down => Direction::Right,
		Direction::Left => Direction::Down,
	}
}

fn turn_right(dir: &Direction) -> Direction {
	match dir {
		Direction::Up => Direction::Right,
		Direction::Right => Direction::Down,
		Direction::Down => Direction::Left,
		Direction::Left => Direction::Up,
	}
}


// fn go_straight(dir: &Direction) -> Direction {
// 	dir.clone()
// }


pub fn solve(inputs : Vec<String>) {
	// let inputs = inputs.iter().map(|line| line.parse::<i32>().unwrap()).collect_vec();
	let mut grid : Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut carts = Vec::new();
	println!("{:?}", grid[0]);

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let ch = grid[r][c];
			if ch == '<' {
				carts.push( Cart{ r: r, c: c, direction: Direction::Left, turn_preference: 0});
				grid[r][c] = '-';
			} else if ch == '>' {
				carts.push( Cart{ r: r, c: c, direction: Direction::Right, turn_preference: 0});
				grid[r][c] = '-';
			} else if ch == '^' {
				carts.push( Cart{ r: r, c: c, direction: Direction::Up, turn_preference: 0});
				grid[r][c] = '|';
			} else if ch == 'v' {
				carts.push( Cart{ r: r, c: c, direction: Direction::Down, turn_preference: 0});
				grid[r][c] = '|';
			}
		}
	}

	for iteration in 0.. {
		println!("Iteration {}: {}", iteration, carts.len());

		// println!("{:?}", carts);

		let mut positions = HashSet::new();
		// for cart in &carts {
		// 	if !positions.insert((cart.r, cart.c)) {
		// 		println!("Collision at {} ({}, {})", iteration, cart.c, cart.r);
		// 		return;
		// 	}
		// }
		carts.sort_by_key(|cart| (cart.r, cart.c));
		
		carts = carts.into_iter().filter(|cart| positions.insert((cart.r, cart.c))).collect_vec();

		if carts.len() == 1 {
			println!("Part 2: {:?}", carts[0]);
			break;
		}

		// for r in 0..grid.len() {
		// 	for c in 0..grid[r].len() {
		// 		if positions.contains(&(r, c)) {
		// 			print!("X");
		// 		} else {
		// 			print!("{}", grid[r][c]);
		// 		}
		// 	}
		// 	println!("");
		// }

		for cart in &mut carts {
			cart.c = match cart.direction {
				Direction::Up | Direction::Down => cart.c,
				Direction::Left => cart.c - 1,
				Direction::Right => cart.c + 1,
			};

			cart.r = match cart.direction {
				Direction::Left | Direction::Right => cart.r,
				Direction::Up => cart.r - 1,
				Direction::Down => cart.r + 1,
			};

			let ch = grid[cart.r][cart.c];
			if ch == ' ' {
				panic!("OH Crap!");
			} else if ch == '+' {
				if cart.turn_preference == 0 {
					cart.direction = turn_left(&cart.direction);
				} else if cart.turn_preference == 1 {
					//cart.direction = go_straight(&cart.direction);
				} else if cart.turn_preference == 2 {
					cart.direction = turn_right(&cart.direction);
				} 
				cart.turn_preference = (cart.turn_preference + 1) % 3;
			} else if ch == '/' {
				cart.direction = match cart.direction {
					Direction::Up => Direction::Right,
					Direction::Right => Direction::Up,
					Direction::Down => Direction::Left,
					Direction::Left => Direction::Down,
				};
			} else if ch == '\\' {
				cart.direction = match cart.direction {
					Direction::Up => Direction::Left,
					Direction::Left => Direction::Up,
					Direction::Down => Direction::Right,
					Direction::Right => Direction::Down,
				};
			}
		}

		carts = carts.into_iter().filter(|cart| positions.insert((cart.r, cart.c))).collect_vec();
		if carts.len() == 1 {
			println!("Part 2!: {:?}", carts[0]);
			break;
		}

		// for cart in &carts {
		// 	if !positions.insert((cart.r, cart.c)) {
		// 		println!("Collision at {} ({}, {})", iteration, cart.c, cart.r);
		// 		return;
		// 	}
		// }
	}
}