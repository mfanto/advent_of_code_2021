use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Width of the binary string in the input
const BINARY_WIDTH: usize = 12;

/// Main function
fn main() {
	match read_lines("./input.txt") {
		Ok(lines)	=> calculate_power(lines),
		Err(e) 	=> eprintln!("There was an error: {}", e)
	}
}

/// Begin the power consumption
fn calculate_power(lines: io::Lines<io::BufReader<File>>) {

	// keep track of the zeros and ones count as we loop through every line
	let mut ones: [i32; BINARY_WIDTH] = [0; BINARY_WIDTH];
	let mut zeros: [i32; BINARY_WIDTH] = [0; BINARY_WIDTH];

	for line in lines {

		// for each line, iterate over the string
		for (index, bit) in line.unwrap().chars().enumerate() {
			if bit == '0' {
				zeros[index] += 1;
			} else {
				ones[index] += 1;
			}
		}
	}

	let mut gamma = "".to_string();
	let mut epsilon = "".to_string();

	// now that we have the count of zero and one at each position
	// we loop over each position, and calculate gamma and epsilon
	for i in 0..BINARY_WIDTH {
		if ones[i] > zeros[i] {
			gamma.push_str("1");
			epsilon.push_str("0");
		} else {
			gamma.push_str("0");
			epsilon.push_str("1");
		}
	}

	// now convert the string to decimal
	let gamma_decimal = isize::from_str_radix(&gamma, 2).unwrap();
	let epislon_decimal = isize::from_str_radix(&epsilon, 2).unwrap();

	println!("Gamma: {}, Gamma Decimal: {}, Epislon: {}, Epislon Decimal: {}, Power: {}",
		gamma, gamma_decimal, epsilon, epislon_decimal, gamma_decimal * epislon_decimal)
}

/// Read a file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
