fn reduce(mut input: Vec<char>) -> Vec<char> {
	for i in (0..input.len()).rev() {
		if i < input.len() - 1 && input[i].eq_ignore_ascii_case(&input[i+1]) && input[i] != input[i+1] {
			input.remove(i);
			input.remove(i);
		}
	}
	return input;
}

pub fn solve(inputs : Vec<String>) {
	let input : Vec<char> = inputs[0].chars().collect();

	let input = reduce(input.clone());
	println!("Part 1: {} ", input.len());

	// Start part 2 with the already reduced part 1 result to save redundant processing in part2
	let mut min_len = input.len();
	for i in 0..26 {
		let ch = (('a' as u8) + i) as char; // Character to remove
		let filtered : Vec<char> = input.iter().filter(|&c| !c.eq_ignore_ascii_case(&ch)).cloned().collect();
		let reduced = reduce(filtered);
		min_len = std::cmp::min(min_len, reduced.len());
	}

	println!("Part 2: {}", min_len);
}