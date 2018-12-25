use itertools::Itertools;
use std::collections::BinaryHeap;
use regex::Regex;
use std::cmp::{Ordering, min};

#[derive(Debug)]
struct Bot {
	x: i64,
	y: i64,
	z: i64,
	r: i64,
}

impl Bot {
	fn dist_to(&self, other: &Self) -> i64 {
		(self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
	}
}

// Fast and dirty implementation of Bron Kerbosch algorithm for finding maximal cliques
fn bron_kerbosch_2(r : Vec<usize>, mut p : Vec<usize>, mut x : Vec<usize>, adj: &Vec<Vec<bool>>, max_clique: &mut Vec<usize>) {
	if p.is_empty() && x.is_empty() {
		if r.len() > max_clique.len() {
			*max_clique = r;
		}
		return;
	}

	if p.is_empty() {
		return;
	}

	let p_clone = p.clone();
	let u = p_clone.first().unwrap().clone(); // pivot

	for v in p_clone {
		if u != v && adj[u][v] { // Skip neighbors of our pivot
			continue;
		}

		let mut r_prime = r.clone();
		let mut p_prime = p.clone();
		let mut x_prime = x.clone();
		r_prime.push(v);
		p_prime.retain(|&pv| v != pv && adj[v][pv]);
		x_prime.retain(|&xv| v != xv && adj[v][xv]);
		bron_kerbosch_2(r_prime, p_prime, x_prime, adj, max_clique);

		p.retain(|&pv| pv != v);
		x.push(v);
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
	bots : usize,
	min : (i64, i64, i64),
	max : (i64, i64, i64),
}

impl Cube {
	fn new(min: (i64, i64, i64), max: (i64, i64, i64), bots: &Vec<Bot>) -> Cube {
		let mut cube = Cube{min, max, bots: 0};
		for bot in bots {
			if cube.intersects_bot(bot) {
				cube.bots += 1;
			}
		} 
		cube
	}

	fn dist_to_origin(&self) -> i64 {
		min(self.min.0.abs(), self.max.0.abs()) +
		min(self.min.1.abs(), self.max.1.abs()) +
		min(self.min.2.abs(), self.max.2.abs())
	}

	fn dist_to_bot(&self, bot: &Bot) -> i64 {
		let mut dist = 0;
		if bot.x < self.min.0 { dist += self.min.0 - bot.x; }
		else if bot.x >= self.max.0 { dist += bot.x - (self.max.0 - 1); }
		if bot.y < self.min.1 { dist += self.min.1 - bot.y; }
		else if bot.y >= self.max.1 { dist += bot.y - (self.max.1 - 1); }
		if bot.z < self.min.2 { dist += self.min.2 - bot.z; }
		else if bot.z >= self.max.2 { dist += bot.z - (self.max.2 - 1); }
		dist
	}

	fn intersects_bot(&self, bot: &Bot) -> bool {
		self.dist_to_bot(bot) <= bot.r
	}

	fn size(&self) -> i64 {
		self.max.0 - self.min.0
	}
}

impl Ord for Cube {
	fn cmp(&self, other: &Self) -> Ordering {
		self.bots.cmp(&other.bots).then(self.size().cmp(&other.size()).reverse()).then(self.dist_to_origin().cmp(&other.dist_to_origin()).reverse())
	}
}

impl PartialOrd for Cube {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn solve_cubes(bots: &Vec<Bot>) -> (i64,i64,i64) {
	let l = -2i64.pow(30);
	let u = 2i64.pow(30);
	let mut heap = BinaryHeap::new();
	heap.push(Cube::new((l, l, l), (u, u, u), bots));

	while !heap.is_empty() {
		let cube = heap.pop().unwrap();
		if cube.size() == 1 {
			println!("Found: {:?}.  Queue.len = {}", cube, heap.len());
			return cube.min;
		}

		let min = cube.min;
		let max = cube.max;
		let mid = ((cube.min.0 + cube.max.0) / 2, (cube.min.1 + cube.max.1) / 2, (cube.min.2 + cube.max.2) / 2);
		heap.push(Cube::new((min.0, min.1, min.2), (mid.0, mid.1, mid.2), bots));
		heap.push(Cube::new((mid.0, min.1, min.2), (max.0, mid.1, mid.2), bots));
		heap.push(Cube::new((min.0, mid.1, min.2), (mid.0, max.1, mid.2), bots));
		heap.push(Cube::new((mid.0, mid.1, min.2), (max.0, max.1, mid.2), bots));
		heap.push(Cube::new((min.0, min.1, mid.2), (mid.0, mid.1, max.2), bots));
		heap.push(Cube::new((mid.0, min.1, mid.2), (max.0, mid.1, max.2), bots));
		heap.push(Cube::new((min.0, mid.1, mid.2), (mid.0, max.1, max.2), bots));
		heap.push(Cube::new((mid.0, mid.1, mid.2), (max.0, max.1, max.2), bots));
	}

	unreachable!()
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
	let mut bots = Vec::new();

	for input in inputs {
		let caps = re_input.captures(&input).unwrap();
		let x = caps[1].parse::<i64>().unwrap();
		let y = caps[2].parse::<i64>().unwrap();
		let z = caps[3].parse::<i64>().unwrap();
		let r = caps[4].parse::<i64>().unwrap();

		bots.push(Bot{x, y, z, r});
	}

	let biggest_r_bot = bots.iter().max_by_key(|bot| bot.r).unwrap();
	let part1 = bots.iter().filter(|bot| bot.dist_to(biggest_r_bot) <= biggest_r_bot.r).count();
	println!("Part 1: {}", part1);

	let mut adjacent = vec![vec![false; bots.len()]; bots.len()];
	for i in 0..bots.len() {
		let bot1 = &bots[i];
		for j in 0..bots.len() {
			let bot2 = &bots[j];
			if bot1.dist_to(&bot2) <= bot1.r + bot2.r {
				adjacent[i][j] = true;
				adjacent[j][i] = true;
			}
		}
	}

	// Look for the cube in the clique which is furthest from the origin.  Its closes point is likely the answer
	let mut maximal_clique = vec![];
	bron_kerbosch_2(vec![], (0..bots.len()).collect_vec(), vec![], &adjacent, &mut maximal_clique);
	let max_dist_bot = maximal_clique.iter().map(|&i| {
		let bot = &bots[i];
		bot.x.abs() + bot.y.abs() + bot.z.abs() - bot.r // distance from origin
	}).max().unwrap();
	println!("Part 2 (Clique approach): Clique size = {}, Furthest bot distance = {}", maximal_clique.len(), max_dist_bot);

	// OctTree
	let part2 = solve_cubes(&bots);
	println!("Part 2 (OctTree approach): {}  {:?}", part2.0.abs() + part2.1.abs() + part2.2.abs(), part2);
}