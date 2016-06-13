extern crate rand;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use rand::Rng;


fn get_prefix(prefixes: & VecDeque<&str>) -> String {
	prefixes.iter()
		.fold(String::new(), |mut prefix_string, &pref| {
			prefix_string.push_str(pref);
			prefix_string.push(' ');
			prefix_string
		})
}

fn select_word<'a, 'b: 'a>(map: &'a HashMap<String, Vec<&'b str>>, prefixes: &mut VecDeque<&'a str>) -> &'b str {
	let key = get_prefix(&prefixes);
	let mut rng = rand::thread_rng();
	let word = match map.get(&key) {
		Some(word) => rng.choose(&word).unwrap(),
		None => {
			// pick a random index in hash for word
			let mut index: usize = rng.gen();
			index = index % map.len();
			let pair: (&'a String, &'a Vec<&'b str>) = map.iter().nth(index).unwrap();
			for prefix in pair.0.split_whitespace() {
				prefixes.push_back(prefix);
			}
			rng.choose(&pair.1).unwrap()
		}
	};
	prefixes.pop_front();
	prefixes.push_back(word);
	word
}

fn main() {
	const PREFIX_SIZE: usize = 2;
	const STORY_SIZE: usize = 500;

	let mut stdin = io::stdin();
	let mut text = String::from("");
	stdin.read_to_string(&mut text).unwrap();

	let mut map: HashMap<String, Vec<&str>> = HashMap::new();
	let mut prefixes: VecDeque<&str> = VecDeque::new();
	{
		for word in text.split_whitespace() {
			if prefixes.len() == PREFIX_SIZE {
				let key = get_prefix(&prefixes);
				if !map.contains_key(&key) {
					map.insert(key.clone(), Vec::new());
				}
				map.get_mut(&key).unwrap()
					.push(word);
				prefixes.pop_front();
			}
			prefixes.push_back(word);
		}

		prefixes.clear();

		for iteration in 0..STORY_SIZE {
			print!("{} ", select_word(& map, &mut prefixes));
		}
	}
}