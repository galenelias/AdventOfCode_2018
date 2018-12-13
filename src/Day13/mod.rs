use std::cell::RefCell;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Direction {
	Left,Up,Right,Down
}

#[derive(Debug, Clone)]
struct Cart {
	r: usize,
	c: usize,
	direction: Direction,
	turn_preference: usize, // left, straight, right
	is_alive: bool,
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
	// (dir + 1) % 4
	match dir {
		Direction::Up => Direction::Right,
		Direction::Right => Direction::Down,
		Direction::Down => Direction::Left,
		Direction::Left => Direction::Up,
	}
}

fn sub_solve(grid : &Vec<Vec<char>>, mut carts: Vec<RefCell<Cart>>, stop_on_first_collision: bool) -> Option<(usize, usize)> {
	for _iteration in 0.. {
		carts.sort_by_key(|cart| (cart.borrow().r, cart.borrow().c));
		
		for (i, cart) in carts.iter().enumerate() {
			let mut cart = cart.borrow_mut();
			if !cart.is_alive {
				continue;
			}

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
			if ch == '+' {
				if cart.turn_preference == 0 {
					cart.direction = turn_left(&cart.direction);
				} else if cart.turn_preference == 1 {
					// no-op
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
			} else if ch == ' ' {
				panic!("Off the rails!");
			}

			// Check for collisions
			for (j, other_cart) in carts.iter().enumerate() {
				// Can't borrow ourselves, so make sure we don't call .borrow if our indices are the same
				if i == j {
					continue;
				}
				let mut other_cart = other_cart.borrow_mut();
				if other_cart.is_alive && cart.r == other_cart.r && cart.c == other_cart.c {
					if stop_on_first_collision {
						return Some((cart.c, cart.r));
					}

					cart.is_alive = false;
					other_cart.is_alive = false;
				}
			}
		}

		// Remove dead carts
		carts = carts.into_iter().filter(|cart| cart.borrow().is_alive).collect_vec();

		if carts.len() == 1 {
			let last_cart = carts[0].borrow();
			return Some((last_cart.c, last_cart.r));
		}
	}

	None
}

pub fn solve(inputs : Vec<String>) {
	let grid : Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();
	let mut carts = Vec::new();

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let ch = grid[r][c];
			if ch == '<' || ch == '>' || ch == 'v' || ch == '^' {
				let dir = match ch {
					'<' => Direction::Left,
					'>' => Direction::Right,
					'^' => Direction::Up,
					'v' => Direction::Down,
					_ => unreachable!(),
				};
				carts.push(RefCell::new(Cart{ r: r, c: c, direction: dir, turn_preference: 0, is_alive: true}));
			}
		}
	}

	let part1 = sub_solve(&grid, carts.clone(), true /*stop_on_first_collision*/);
	println!("Part 1: {:?}", part1.expect("Part 1 failed"));

	let part2 = sub_solve(&grid, carts, false /*stop_on_first_collision*/);
	println!("Part 2: {:?}", part2.expect("Part 2 failed"));
}