fn main() {
	const ENTRIES: i32 = 101;
	let place = std::env::args()
					.nth(1).expect("Needs a input")
					.parse::<i32>().expect("parsed");
	
	for entry in (1..ENTRIES).filter(|&entry| entry != place) {
		printPlace(entry);
	}
}

fn printPlace(place: i32) {
	let ending = match place {
		11...13 => "th",
		num @ _ => {
			match num % 10 {
				1 => "st",
				2 => "nd",
				3 => "rd",
				_ => "th"
			}
		}
	};
	println!("{0}{1}", place, ending);
}