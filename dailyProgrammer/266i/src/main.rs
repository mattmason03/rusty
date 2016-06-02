use std::io;
use std::io::BufRead;
use std::fmt;
use std::ops::{Index, IndexMut};

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();

	let node_count = lines.nth(0)
						.unwrap().unwrap()
						.parse::<usize>()
						.unwrap();

	let mut adjacency_matrix = AdjacencyMatrix::new(36);

	for line in lines {
		let directed: Vec<usize> = line.unwrap().split(" ")
			.map(|input| input.parse::<usize>().unwrap() - 1)
			.collect();

		adjacency_matrix[(directed[0], directed[1])] += 1;
	}

	println!("{}", adjacency_matrix);
}

struct AdjacencyMatrix {
	matrix: Vec<i32>,
	len: usize
}

impl AdjacencyMatrix {
	fn new(len: usize) -> AdjacencyMatrix {
		AdjacencyMatrix {
			matrix: vec![0; len * len],
			len: len
		}
	}
}

impl Index<(usize, usize)> for AdjacencyMatrix {
	type Output = i32;

	fn index<'a>(&'a self, _index: (usize, usize)) 
	-> &'a i32 {
		& self.matrix[self.len * _index.1 + _index.0]
	}
}

impl IndexMut<(usize, usize)> for AdjacencyMatrix {
	fn index_mut<'a>(&'a mut self, _index: (usize, usize))
	-> &'a mut i32 {
		&mut self.matrix[self.len * _index.1 + _index.0]
	}
}

impl fmt::Display for AdjacencyMatrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		for column in 0..self.len {
			try!(write!(f,"["));
			for row in 0..self.len {
				try!(write!(f, "{}", self[(column, row)]));
			}
			try!(write!(f,"]\n"));
		}
		write!(f,"")
	}
}