use itertools::Itertools;

struct Node {
	children: Vec<Node>,
	metadata: Vec<usize>,
}

fn parse_node(input: &Vec<usize>, pos: &mut usize) -> Node {
	let child_count = input[*pos];
	let metadata_count = input[*pos + 1];
	(*pos) += 2;

	let mut node = Node { children: vec![], metadata: vec![]};

	for _ in 0..child_count {
		node.children.push(parse_node(input, pos));
	}

	for _ in 0..metadata_count {
		node.metadata.push(input[*pos]);
		(*pos) += 1;
	}

	return node;
}

fn sum_metadata(node: &Node) -> usize {
	node.metadata.iter().sum::<usize>()
		+ node.children.iter().map(|child| sum_metadata(child)).sum::<usize>()
}

fn value_of(node: &Node) -> usize {
	if node.children.is_empty() {
		node.metadata.iter().sum()
	} else {
		node.metadata.iter().map(|m| {
			if m - 1 < node.children.len() {
				value_of(&node.children[m-1])
			} else {
				0
			}
		}).sum()
	}
}

pub fn solve(inputs : Vec<String>) {
	let input = inputs[0].split(" ").map(|w| w.parse::<usize>().unwrap()).collect_vec();

	let mut pos = 0;
	let root = parse_node(&input, &mut pos);

	println!("Part 1: {}", sum_metadata(&root));
	println!("Part 2: {}", value_of(&root));
}