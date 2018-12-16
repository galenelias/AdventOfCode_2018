use std::collections::HashSet;
use itertools::Itertools;

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<i32>().unwrap()).collect_vec();

	let part1 = inputs.iter().sum::<i32>();
	println!("Part 1: {}", part1);

	let mut vals = HashSet::new();
	let part2 = inputs.iter().cycle().try_fold(0, |sum, x| {
		if !vals.insert(sum) { Err(sum) } else { Ok(sum + x) }
	}).unwrap_err();

	println!("Part 2: {}", part2);
}