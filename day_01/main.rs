use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Main function
fn main() {
	match read_lines("./input.txt") {
		Ok(lines) => calculate_increases(lines),
		Err(e) => eprintln!("There was an error: {}", e)
	}
}

/// Begin the calculation
fn calculate_increases(lines: io::Lines<io::BufReader<File>>) {

	let mut increases = 0;
	let mut last_value: Option<i32> = None;

	for line in lines {

		if let Ok(amount_str) = line {

			let amount = amount_str.parse::<i32>().unwrap();

			if does_increase(amount, last_value) {
				increases += 1;
			}

			last_value = Some(amount);
		}
	}

	println!("Increased: {}", increases);
}

// determine whether this line is an increase over the last line
fn does_increase(amount: i32, last_line: Option<i32>) -> bool {

	return match last_line {
		Some(x) => if amount > x {true} else {false},
		_ => false
	}
}

/// Read a file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
