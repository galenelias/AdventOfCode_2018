use itertools::Itertools;

fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
	(b[0] - a[0]).abs() + (b[1] - a[1]).abs() + (b[2] - a[2]).abs() + (b[3] - a[3]).abs()
}

// Quick and dirty transitive closure algorithm. Spread adjacencies.
fn transitive_closure(mut adj: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	loop {
		let mut did_work = false;
		for i in 0..adj.len() {
			for j in 0..adj.len() {
				for k in 0..adj.len() {
					if adj[i][j] && adj[j][k] && !adj[i][k] {
						did_work = true;
						adj[i][k] = true;
					}
				}
			}
		}

		if !did_work {
			break;
		}
	}
	adj
}

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.trim().split(",").map(|w| w.parse::<i32>().unwrap()).collect_vec()).collect_vec();
	let mut adj = vec![vec![false; inputs.len()]; inputs.len()];

	for i in 0..inputs.len() {
		for j in 0..inputs.len() {
			let d = dist(&inputs[i], &inputs[j]);
			if d <= 3 {
				adj[i][j] = true;
				adj[j][i] = true;
			}
		}
	}
	let transtive_adj = transitive_closure(adj);

	let mut colors = vec![0; inputs.len()];
	let mut color_num = 0;
	for i in 0..inputs.len() {
		if colors[i] == 0 {
			color_num += 1;
			colors[i] = color_num;
			for j in 0..inputs.len() {
				if transtive_adj[i][j] {
					colors[j] = color_num;
				}
			}
		}
	}
	println!("Part 2: {}", color_num);
}