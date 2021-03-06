use itertools::Itertools;

// Function to see if two collection of chars don't match on a certain number of characters, and returns the string if so, None otherwise
fn check_diff_by(a: &Vec<char>, b: &Vec<char>, diff_amt: usize) -> Option<String> {
	let same_chars = a.iter().zip_eq(b.iter())
		.filter(|(&ca, &cb)| ca == cb)
		.map(|(&c, _)| c)
		.collect_vec();
	if same_chars.len() == a.len() - diff_amt {
		Some(same_chars.iter().collect::<String>())
	} else {
		None
	}
}

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let all_freqs = inputs.iter()
		.map(|line| {
			let mut frequencies = [0u32; 256];
			for &c in line {
				frequencies[c as usize] += 1;
			}
			frequencies
		}).collect_vec();

	let twos = all_freqs.iter().filter(|freqs| freqs.iter().any(|&v| v == 2)).count();
	let threes = all_freqs.iter().filter(|freqs| freqs.iter().any(|&v| v == 3)).count();
	println!("Part 1: {}", twos * threes);

	let part2 = inputs.iter()
		.combinations(2).
		filter_map(|combo| check_diff_by(combo[0], combo[1], 1))
		.next().unwrap();
	println!("Part 2: {}", part2);
}