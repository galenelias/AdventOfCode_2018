
pub fn solve(inputs : Vec<String>) {
	let input = &inputs[0];
	let input_ints = input.chars().filter_map(|c| { return c.to_digit(10);}).collect::<Vec<u32>>();

	let sum1 : u32 = input_ints.iter()
		.zip(input_ints.iter().cycle().skip(1))
		.filter_map(|(a, b)| { if a == b { Some(a) } else { None }})
		.sum();

	let sum2 : u32 = input_ints.iter()
		.zip(input_ints.iter().cycle().skip(input_ints.len()/2))
		.filter_map(|(a, b)| { if a == b { Some(a) } else { None }})
		.sum();

	println!("Part1: {}", sum1);
	println!("Part2: {}", sum2);
}