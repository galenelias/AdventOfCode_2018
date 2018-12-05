use itertools::Itertools;

fn fully_reduce(mut input: Vec<char>) -> Vec<char> {
	loop {
		let mut reduced = false;

		for i in (0..input.len()-1).rev() {
			if input[i].eq_ignore_ascii_case(&input[i+1]) && input[i] != input[i+1] {
				input.remove(i);
				input.remove(i);
				reduced = true;
			}
		}

		if !reduced {
			return input;
		}
	}
}

pub fn solve(inputs : Vec<String>) {
	let input = inputs[0].chars().collect_vec();

	let input = fully_reduce(input.clone());
	println!("Part 1: {} ", input.len());

	// Start part 2 with the already reduced part 1 result to save redundant processing in part2
	let mut min_len = input.len();
	for i in 0..26 {
		let ch = (('a' as u8) + i) as char; // Character to remove

		let filtered = input.iter().cloned().filter(|&c| !c.eq_ignore_ascii_case(&ch)).collect_vec();
		let reduced = fully_reduce(filtered);
		min_len = std::cmp::min(min_len, reduced.len());
	}

	println!("Part 2: {}", min_len);
}