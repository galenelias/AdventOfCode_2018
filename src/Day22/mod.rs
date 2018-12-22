use itertools::Itertools;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Type {
	Rocky,
	Narrow,
	Wet,
}

fn get_risk(region_type: &Type) -> usize {
	match region_type {
		Type::Rocky => 0,
		Type::Wet => 1,
		Type::Narrow => 2,
	}
}

fn get_index(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize,usize),usize>) -> usize {
	if let Some(level) = memo.get(&pos) {
		return *level;
	}
	let level = if pos == (0,0) {
		0
	} else if pos == target {
		return 0
	} else if pos.0 == 0 {
		pos.1 * 16807
	} else if pos.1 == 0 {
		pos.0 * 48271
	} else {
		get_erosion((pos.0 - 1, pos.1), depth, target, memo) * get_erosion((pos.0, pos.1 - 1), depth, target, memo)
	};

	memo.insert(pos, level);
	level
}

fn get_erosion(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize,usize),usize>) -> usize {
	let index = get_index(pos, depth, target, memo);
	(depth + index) % 20183
}

fn get_type(pos: (usize, usize), depth: usize, target: (usize, usize), memo: &mut HashMap<(usize,usize),usize>) -> Type {
	let erosion = get_erosion(pos, depth, target, memo);
	match erosion % 3 {
		0 => Type::Rocky,
		1 => Type::Wet,
		2 => Type::Narrow,
		_ => unreachable!(),
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Equipment {
	Neither,
	Torch,
	Climbing,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct State {
	time: usize,
	min_dist: usize,
	pos: (i32, i32),
	equipped: Equipment,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.time + self.min_dist).cmp(&(other.time + other.min_dist)).reverse()
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn min_dist(pos: (i32, i32), target: (usize, usize)) -> usize {
	((pos.0 - (target.0 as i32)).abs() + (pos.1 - (target.1 as i32)).abs()) as usize
}

fn navigate(target: (usize, usize), depth: usize, type_memo: &mut HashMap<(usize,usize),usize>) -> usize {
	let mut queue : BinaryHeap<State> = BinaryHeap::new();
	let mut seen = HashMap::new();
	queue.push(State{ pos: (0, 0), min_dist: min_dist((0, 0), target), time: 0, equipped: Equipment::Torch});

	while !queue.is_empty() {
		let state = queue.pop().unwrap();
		let region = get_type((state.pos.0 as usize, state.pos.1 as usize), depth, target, type_memo);

		if region == Type::Rocky && state.equipped == Equipment::Neither {
			continue;
		} else if region == Type::Wet && state.equipped == Equipment::Torch {
			continue;
		} else if region == Type::Narrow && state.equipped == Equipment::Climbing {
			continue;
		}

		if seen.get(&(state.pos, state.equipped)).unwrap_or(&usize::max_value()) <= &state.time {
			continue;
		} else {
			seen.insert((state.pos, state.equipped), state.time);
		}

		if state.pos.0 as usize == target.0 && state.pos.1 as usize == target.1 && state.equipped == Equipment::Torch {
			return state.time;
		}

		// Queue up all possible movements
		for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
			let pos = (state.pos.0 - dy, state.pos.1 - dx);
			if pos.0 >= 0 && pos.1 >= 0 {
				queue.push(State{ pos: pos, min_dist: min_dist(pos, target), time: state.time + 1, equipped: state.equipped});
			}
		}

		// Queue up all possible equipment switches
		for equipment in &[Equipment::Neither, Equipment::Torch, Equipment::Climbing] {
			if &state.equipped != equipment {
				queue.push(State{ pos: state.pos, min_dist: state.min_dist, time: state.time + 7, equipped: *equipment});
			}
		}
	}
	unreachable!();
}

pub fn solve(inputs : Vec<String>) {
	let depth = inputs[0].split(": ").skip(1).next().unwrap().parse::<usize>().unwrap();
	let target_vec = inputs[1].split(": ").skip(1).next().unwrap().split(",").map(|w| w.parse::<usize>().unwrap()).collect_vec();
	let target = (target_vec[1], target_vec[0]);

	let mut memo = HashMap::new();
	let mut total_risk = 0;
	for y in 0..=target.0 {
		for x in 0..=target.1 {
			total_risk += get_risk(&get_type((y, x), depth, target, &mut memo));
		}
	}
	println!("Part 1: {}", total_risk);

	let part2 = navigate(target, depth, &mut memo);
	println!("Part 2: {}", part2);
}