use std::io;
use std::io::BufRead;

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();

	let nodeCount = lines.nth(0)
						.unwrap().unwrap()
						.parse::<usize>()
						.unwrap();

	let mut node: Vec<i32> = vec![0; nodeCount];

	for line in lines {
		let adjNodes: Vec<usize> = line.unwrap().split(" ")
			.map(|input| input.parse::<usize>().unwrap() - 1)
			.collect();

		node[adjNodes[0]] += 1;
		node[adjNodes[1]] += 1;
	}

	for index in 0..node.len() {
		println!("Node {} has a degree of {}", index, node[index]);
	}
}
