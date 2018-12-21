use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default, Debug)]
struct Node {
	part: String,
	id: usize,
	children: Vec<Node>,
	next: Option<Box<Node>>,
}

impl Node {
	fn is_empty(&self) -> bool {
		self.part.is_empty() && self.children.is_empty() && self.next.is_none()
	}
}

fn parse_node(input: &Vec<char>, idx: &mut usize, _is_top: bool) -> Node {
	let mut part = String::new();
	let mut node = Node::default();

	node.id = *idx;

	while input[*idx] == 'W' || input[*idx] == 'N' || input[*idx] == 'E' || input[*idx] == 'S' {
		part.push(input[*idx]);
		(*idx) += 1;
	}
	node.part = part;

	if input[*idx] == '(' {
		*idx += 1;
		loop {
			let sub_node = parse_node(input, idx, false);
			node.children.push(sub_node);
			if input[*idx] == ')' {
				(*idx) += 1;
				break;
			}
			(*idx) += 1;
		}
		
		let next = parse_node(input, idx, false);
		if !next.is_empty() {
			node.next = Some(Box::new(next));
		}
	}

	return node;
}

fn move_pos(pos: (i32,i32), mag: i32, dir: char) -> (i32,i32) {
	match dir {
		'N' => (pos.0 - mag, pos.1),
		'S' => (pos.0 + mag, pos.1),
		'W' => (pos.0, pos.1 - mag),
		'E' => (pos.0, pos.1 + mag),
		_ => unreachable!(),
	}
}

fn children_all_loops(node: &Node) -> bool {
	node.children.iter().all(
		|child| child.children.is_empty() && child.next.is_none() && child.part.chars().fold((0,0), |pos, ch| move_pos(pos, 1, ch)) == (0,0)
	)
}

fn do_walk(mut pos: (i32, i32), node: &Node, returns: Vec<&Node>, doors: &mut HashSet<(i32,i32)>) {
	// println!("Do_walk: pos={:?}, node={} ({}), return_stack={}, doors={}", pos, node.id, node.part, returns.len(), doors.len());

	for ch in node.part.chars() {
		doors.insert(move_pos(pos, 1, ch));
		pos = move_pos(pos, 2, ch);
	}

	if !node.children.is_empty() && !children_all_loops(node) {
		for child in &node.children {
			let mut child_returns = returns.clone();
			if node.next.is_some() {
				child_returns.push(node.next.as_ref().unwrap());
			}
			do_walk(pos, &child, child_returns, doors);
		}
	} else {
		// If all our children are loops, we don't need to branch here.  We can visit the loops, then resume processing afterwards.
		for child in &node.children {
			do_walk(pos, &child, vec![], doors);
		}

		let mut child_returns = returns.clone();
		if node.next.is_some() {
			child_returns.push(node.next.as_ref().unwrap());
		}

		if !child_returns.is_empty() {
			let return_node = child_returns.pop().unwrap();
			do_walk(pos, &return_node, child_returns, doors);
		}
	}
}

fn build_distances(pos: (i32, i32), doors: &HashSet<(i32,i32)>) -> HashMap<(i32,i32),usize> {
	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	let mut distances = HashMap::new();

	queue.push_back((pos, 0));

	while !queue.is_empty() {
		let (p, d) = queue.pop_front().unwrap();
		if !visited.insert(p) {
			continue;
		}
		distances.insert(p, d);

		for &dir in &['N','E','W','S'] {
			if doors.contains(&move_pos(p, 1, dir)) {
				queue.push_back((move_pos(p, 2, dir), d + 1));
			}
		}
	}
	distances
}

fn _print_nodes(node: &Node, indent: usize) {
	println!("{}{}, \"{}\", children={}, next={}", "  ".repeat(indent), node.id, node.part, node.children.len(), node.next.as_ref().map_or(0, |node| (*node).id));

	for child in &node.children {
		_print_nodes(child, indent + 1);
	}

	if node.next.is_some() {
		_print_nodes(node.next.as_ref().unwrap(), indent);
	}
}

fn print_grid(doors: &HashSet<(i32,i32)>) {
	let min_y = doors.iter().map(|k| k.0).min().unwrap();
	let max_y = doors.iter().map(|k| k.0).max().unwrap();
	let min_x = doors.iter().map(|k| k.1).min().unwrap();
	let max_x = doors.iter().map(|k| k.1).max().unwrap();

	for y in min_y-1..max_y+2 {
		for x in min_x-1..max_x+2 {
			if y == 0 && x == 0 {
				print!("X");
			} else if (y % 2) == 0 && (x % 2) == 0 {
				print!("·");
			} else {
				if doors.contains(&(y, x)) {
					print!("{}", if (y % 2) == 0 { "|" } else { "-" });
				} else {
					print!("#");
				}
			}
		}
		println!("");
	}
}

pub fn solve(inputs : Vec<String>) {
	for input in inputs {
		// println!("Processing: {}", input);
		let input = input.chars().collect_vec();

		let mut idx = 1;
		let root = parse_node(&input, &mut idx, true);

		// print_nodes(&root, 0);
		let mut doors = HashSet::new();

		do_walk((0, 0), &root, Vec::new(), &mut doors);
		print_grid(&doors);

		let distances = build_distances((0, 0), &doors);

		let part1 = distances.values().max().unwrap();
		println!("Part 1: {:?} ", part1);

		let part2 = distances.values().filter(|&v| v >= &1000).count();
		println!("Part 2: {:?} ", part2);
	}
}