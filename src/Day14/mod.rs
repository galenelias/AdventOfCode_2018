use itertools::Itertools;

pub fn solve(inputs : Vec<String>) {
	let input_num = inputs[0].parse::<usize>().unwrap();
	let input = inputs[0].chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect_vec();

	let mut elves = vec![0, 1];
	let mut scores = vec![3, 7];

	let mut part1 = None;
	let mut part2 = None;

	while part1.is_none() || part2.is_none()  {
		let combo = scores[elves[0]] + scores[elves[1]];

		if combo >= 10 {
			scores.push(combo / 10);
		}
		scores.push(combo % 10);

		for elf in &mut elves {
			*elf = (1 + *elf + scores[*elf]) % scores.len();
		}

		if part1.is_none() && scores.len() >= input_num + 10 {
			part1 = Some(scores[input_num .. input_num + 10].iter().map(|i| i.to_string()).collect::<String>());
		}

		// Part 2, compare the trailing scores to the pattern we're seeking.  Might need to look one addition place back if we appended two scores this round
		let len = scores.len();
		if part2.is_none() && len >= input.len() && scores[len-input.len()..] == input[..] {
			part2 = Some(len - input.len());
		} else if part2.is_none() && len >= input.len() + 1 && scores[len-input.len()-1 .. len-1] == input[..] {
			part2 = Some(len - input.len() - 1);
		}
	}
	
	println!("Part 1: {}", part1.unwrap());
	println!("Part 2: {}", part2.unwrap());
}