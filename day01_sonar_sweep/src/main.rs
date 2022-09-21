use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process;

fn main() {
    // Get input file from command line.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
        process::exit(1);
    }
    let input_path = Path::new(&args[1]);

    // Open file read-only.
    let file = match File::open(&input_path) {
        Err(msg) => panic!("Could not open file for reading: {} ({})", input_path.display(), msg),
        Ok(f) => f
    };

    // Keep track of the most recent value and the number of adjacent value increases.
    let mut optional_prev_value: Option<i32> = None;
    let mut increase_count: i32 = 0;

    // Read the file line by line.
    let line_reader = BufReader::new(&file).lines();
    for line_result in line_reader {
        let line = line_result.expect("Failed to read line");

        // Parse integer value from the line.
        let cur_value: i32 = match line.parse() {
            Err(_) => panic!("Could not parse value from line: {}", &line),
            Ok(val) => val
        };

        // Compare to previous value (if present).
        if let Some(prev_value) = optional_prev_value {
            if prev_value < cur_value {
                increase_count += 1;
            }
        }

        // Current value is now the previous value.
        optional_prev_value = Some(cur_value);
    }

    println!("Number of increases: {}", increase_count);
}
