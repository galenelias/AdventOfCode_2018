use itertools::Itertools;
use std::collections::HashMap;

fn state_to_string(state: &HashMap<i32, bool>) -> String {
	let min_index = state.keys().cloned().min().unwrap();
	let max_index = state.keys().cloned().max().unwrap();

	let mut result = String::new();
	for x in min_index..=max_index {
		result.push(if *state.get(&x).unwrap_or(&false) { '#' } else { '.' });
	}
	result
}

fn get_total(state: &HashMap<i32, bool>, bias_shift: i64) -> i64 {
	let mut total = 0;
	for (&k, &v) in state {
		if v {
			total += k as i64 + bias_shift;
		}
	}
	total
}

pub fn solve(inputs : Vec<String>) {
	let initial_state = inputs[0][(inputs[0].find(": ").unwrap() + 2)..].to_string();
	let mut state : HashMap<i32, bool> = HashMap::new();

	for (i, c) in initial_state.chars().enumerate() {
		if c == '#' {
			state.insert(i as i32, true);
		}
	}

	let rules_parts = inputs[2..].iter().map(|line| line.split(" => ").map(|w| w.to_string()).collect_vec()).collect_vec();

	let mut rules = HashMap::new();
	for r in rules_parts {
		rules.insert(r[0].clone(), r[1].chars().next().unwrap());
	}

	let mut state_str = state_to_string(&state);
	for iter in 0.. {
		let mut new_state = HashMap::new();

		let min_index = state.keys().cloned().min().unwrap();
		let max_index = state.keys().cloned().max().unwrap();

		for index in (min_index - 2)..=(max_index + 2) {
			let mut key = String::new();

			for i in -2i32..=2i32 {
				if *state.get(&(index + i)).unwrap_or(&false) {
					key.push('#');
				} else {
					key.push('.');
				}
			}

			let res = rules.get(&key).unwrap_or(&'.');

			if res == &'#' {
				new_state.insert(index, true);
			}
		}

		if iter == 19 {
			println!("Part 1: {}", get_total(&new_state, 0));
		}

		let new_state_str = state_to_string(&new_state);
		if state_str == new_state_str {
			let new_min_index = *new_state.keys().min().unwrap();
			// println!("Duplicate after {}, shifting by {}", iter, new_min_index - min_index);
			println!("Part 2: {}", get_total(&state, 50000000000i64 - iter));
			break;
		}

		state = new_state;
		state_str = new_state_str;
	}
}