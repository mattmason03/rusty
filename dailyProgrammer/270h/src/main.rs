use std::io;
use std::io::Read;
use std::cmp;

fn build_prefix_sums(size: u32, world: &mut Vec<(char, u32)>, point: (u32, u32)) -> u32 {
	let idx = point.0 + size * point.1;
	let terrain = world[idx as usize].0;
	let largest_square = match terrain {
		'X' => 0,
		_ => {
			let l = world[(idx - 1) as usize].1;
			let u = world[(idx - size) as usize].1;
			let ul = world[(idx - size - 1) as usize].1;
			cmp::min(cmp::min(l, u), ul) + 1
		}
	};
	world[idx as usize] = (terrain, largest_square);
	largest_square
}

fn main() {
	let mut stdin = io::stdin();

	let mut input = String::new();
	stdin.read_to_string(&mut input).unwrap();

	let mut chars = input.chars();
	let size = chars.nth(0).unwrap()
						.to_digit(10).unwrap();

	let mut world: Vec<(char, u32)> = 
		chars.filter(|&letter| letter == 'X' || letter == '-')
		.map(|letter| {
			let val = match letter {
				'X' => 0,
				_ => 1
			};
			(letter, val)
		})
		.collect();

	let mut largest_square = 0;
	for diag in 1..size {
		for offset in diag..size {
			let x = build_prefix_sums(size, &mut world, (diag, offset));
			let y = build_prefix_sums(size, &mut world, (offset, diag));
			let max = cmp::max(x, y);
			largest_square = cmp::max(largest_square, max);
		}
	}

	let mut worldIter = world.iter();
	for y in 0..size {
		for x in 0..size {
			let idx= x + y * size;
			print!("{}", world[idx as usize].0);
		}
		println!("");
	}

	for y in 0..size {
		for x in 0..size {
			let idx= x + y * size;
			print!("{}", world[idx as usize].1);
		}
		println!("");
	}

	print!("{} is the largest square", largest_square);
}
