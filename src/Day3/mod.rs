use itertools::Itertools;
use regex::Regex;

struct Claim {
	num: u32,
	left: usize,
	top: usize,
	width: usize,
	height: usize,
}

pub fn solve(inputs : Vec<String>) {
	let re_claim = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
	let mut fabric = vec![vec![0u32; 1000]; 1000];

	let claims = inputs.iter().map(|claim| {
		let caps = &re_claim.captures(&claim).unwrap();
		Claim {
			num: caps[1].parse::<u32>().unwrap(),
			left: caps[2].parse::<usize>().unwrap(),
			top: caps[3].parse::<usize>().unwrap(),
			width: caps[4].parse::<usize>().unwrap(),
			height: caps[5].parse::<usize>().unwrap(),
		}
	}).collect_vec();

	for claim in &claims {
		for r in claim.top..(claim.top + claim.height) {
			for c in claim.left..(claim.left + claim.width) {
				fabric[r as usize][c as usize] += 1;
			}
		}
	}

	// Count all squares with a usage of > 1  (sum of filtered counts)
	let part1 : usize = fabric.iter()
		.map(|row| row.iter().filter(|&c| c > &1).count())
		.sum();
	println!("Part 1: {}", part1);

	// Find first claim whose fabric squares only have a single usage
	let part2 = claims.iter()
		.filter(|claim| {
			fabric[claim.top..(claim.top + claim.height)].iter()
				.all(|row| row[claim.left..(claim.left + claim.width)].iter()
					.all(|&c| c == 1))
		})
		.map(|claim| claim.num)
		.next().unwrap();

	println!("Part 2: {}", part2);
}