fn run_game(players: usize, last_marble_points: usize) {
	let mut marbles = vec![0; 1];
	let mut scores = vec![0u64; players];
	let mut current: i64 = 0;

	marbles.reserve(last_marble_points);

	for marble_number in 1..=last_marble_points {
		let mut insert_point = (current + (marbles.len() as i64) + 2) % (marbles.len() as i64);
		if insert_point == 0 {
			insert_point = marbles.len() as i64;
		}

		if marble_number % 23 == 0 {
			let other_marble = (insert_point + (marbles.len() as i64) - 9) % (marbles.len() as i64);
			scores[(marble_number - 1) % players] += (marble_number + marbles[other_marble as usize]) as u64;
			marbles.remove(other_marble as usize);
			current = other_marble % (marbles.len() as i64);
		} else {
			marbles.insert(insert_point as usize, marble_number);
			current = insert_point;
		}
	}

	println!("Players: {}\tLast Marble: {}\tHigh Score: {}", players, last_marble_points, scores.iter().max().unwrap());
}

pub fn solve(_inputs : Vec<String>) {
	run_game(455, 71223);
	run_game(455, 7122300);
}