use std::collections::HashSet;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<i32>().unwrap()).collect_vec();

	let part1 = inputs.iter().sum::<i32>();
	println!("Part 1: {}", part1);

	let mut vals = HashSet::new();
	let part2 = inputs.iter().cycle().fold_while(0, |sum, x| {
		if !vals.insert(sum) { Done(sum) } else { Continue(sum + x) }
	}).into_inner();

	println!("Part 2: {}", part2);
}