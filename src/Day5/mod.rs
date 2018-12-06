fn reduce(mut input: Vec<char>) -> Vec<char> {
	for i in (0..input.len()).rev() {
		if i < input.len() - 1 && input[i] != input[i+1] && input[i].eq_ignore_ascii_case(&input[i+1]) {
			input.remove(i);
			input.remove(i);
		}
	}
	return input;
}

pub fn solve(inputs : Vec<String>) {
	let input : Vec<char> = inputs[0].chars().collect();

	let input = reduce(input);
	println!("Part 1: {} ", input.len());

	// Start part 2 with the already reduced part 1 result to save redundant processing in part2
	let part2 = (0..26).map(|i| {
		let ch = (('a' as u8) + i) as char; // Character to remove
		let filtered : Vec<char> = input.iter().filter(|&c| !c.eq_ignore_ascii_case(&ch)).cloned().collect();
		reduce(filtered).len()
	}).min().unwrap();

	println!("Part 2: {}", part2);
}