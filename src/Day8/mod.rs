use itertools::Itertools;

#[derive(Default, Debug)]
struct Node {
	children: Vec<Node>,
	metadata: Vec<usize>,
}

fn parse_node<'a>(vals: &mut impl Iterator<Item = usize>) -> Node
{
	let child_count = vals.next().unwrap();
	let metadata_count = vals.next().unwrap();

	let mut node = Node::default();

	for _ in 0..child_count {
		node.children.push(parse_node(vals));
	}

	for _ in 0..metadata_count {
		node.metadata.push(vals.next().unwrap());
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
	let root = parse_node(&mut input.into_iter());

	println!("Part 1: {}", sum_metadata(&root));
	println!("Part 2: {}", value_of(&root));
}