use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Main function
fn main() {
	match read_lines("./directions.txt") {
		Ok(lines)	=> run_ship(lines),
		Err(e) 	=> eprintln!("There was an error: {}", e)
	}
}

/// Run the ship according to a list of commands
/// The commands are lines from the input file
fn run_ship(lines: io::Lines<io::BufReader<File>>) {

	let mut position = (0, 0);

	for line in lines {

		position = match line {
			Ok(command) => move_ship(&command, position),
			Err(e) => panic!("Invalid line. We've crashed {}", e)
		}
	}

	display_position(position);
}

/// Move the submarine according to a command, and the current position
/// Returns the new position of the ship
fn move_ship(command: &str, position: (i32, i32)) -> (i32, i32) {

	let parts: Vec<_> = command.split_whitespace().collect();

	// be explicit about variable names
	let direction = parts[0];
	let amount = parts[1].parse::<i32>().unwrap();
	let (horizontal, depth) = position;

	return match direction {
		"forward" => (horizontal + amount, depth),
		"up" => (horizontal, depth - amount),
		"down" => (horizontal, depth + amount),
		_ => panic!("We haven't been trained in this direction")
	}
}

/// Display the position of the submarine
fn display_position(position: (i32, i32)) {

	let (horizontal, depth) = position;

	println!("Horizontal: {}, Depth: {}, Total: {}", horizontal, depth, horizontal * depth);
}

/// Read a file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
