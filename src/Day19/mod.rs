use itertools::Itertools;

struct Instruction {
	opcode: String,
	input: Vec<usize>,
}

fn run_instr(inst: &Instruction, regs: &mut [usize]) {
	regs[inst.input[2]] = match inst.opcode.as_ref() {
		"addr" => regs[inst.input[0]] + regs[inst.input[1]], //addr
		"addi" => regs[inst.input[0]] + inst.input[1], //addi
		"mulr" => regs[inst.input[0]] * regs[inst.input[1]], //mulr
		"muli" => regs[inst.input[0]] * inst.input[1], //muli
		"banr" => regs[inst.input[0]] & regs[inst.input[1]], //banr
		"bani" => regs[inst.input[0]] & inst.input[1], //bani
		"borr" => regs[inst.input[0]] | regs[inst.input[1]], //borr
		"bori" => regs[inst.input[0]] | inst.input[1], //bori
		"setr" => regs[inst.input[0]], //setr
		"seti" => inst.input[0], //seti
		"gtir" => if inst.input[0] > regs[inst.input[1]] { 1 } else { 0 }, //gtir
		"gtri" => if regs[inst.input[0]] > inst.input[1] { 1 } else { 0 }, //gtri
		"gtrr" => if regs[inst.input[0]] > regs[inst.input[1]] { 1 } else { 0 }, //gtrr
		"eqir" => if inst.input[0] == regs[inst.input[1]] { 1 } else { 0 }, //eqir
		"eqri" => if regs[inst.input[0]] == inst.input[1] { 1 } else { 0 }, //eqri
		"eqrr" => if regs[inst.input[0]] == regs[inst.input[1]] { 1 } else { 0 }, //eqrr
		_ => unreachable!(),
	};
}

pub fn solve(inputs : Vec<String>) {
	let ip_reg = inputs[0].split(" ").skip(1).next().unwrap().parse::<usize>().unwrap();
	let inputs = inputs.iter().skip(1).map(|line| line.split(" ").collect_vec()).collect_vec();

	let program = inputs.iter().map(|line| {
		let opcode = line[0].to_string();
		let input = line.iter().skip(1).map(|w| w.parse::<usize>().unwrap()).collect_vec();
		Instruction{ opcode, input}
	}).collect_vec();

	let mut regs = vec![0usize; 6];
	while regs[ip_reg] < inputs.len() {
		run_instr(&program[regs[ip_reg]], &mut regs);
		regs[ip_reg] += 1;
	}
	println!("Part 1: {}  (full registers = {:?}", regs[0], regs);

	// Part 2 - sum factors of 'r2' which is derived from your puzzle input after running the init code (which eventually returns back to location 2)
	let mut regs = vec![1usize, 0, 0, 0, 0, 0];
	while regs[ip_reg] != 2 {
		run_instr(&program[regs[ip_reg]], &mut regs);
		regs[ip_reg] += 1;
	}

	let part2 = (1..=regs[2]).filter(|x| (regs[2] % x) == 0).sum::<usize>();
	println!("Part 2: {}", part2);
}