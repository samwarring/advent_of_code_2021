struct SonarSweep {
    increase1_count: i32, // Number of increases (line by line)
    increase3_count: i32, // Number of increases (sliding window)
    prev_values: [i32; 3]
}

impl SonarSweep {
    fn new(val1: i32, val2: i32, val3: i32) -> Self {
        let mut initial_count = 0;
        if val1 < val2 {
            initial_count += 1;
        }
        if val2 < val3 {
            initial_count += 1;
        }
        Self {
            increase1_count: initial_count,
            increase3_count: 0,
            prev_values: [val1, val2, val3]
        }
    }

    fn sweep(&mut self, val: i32) {
        // If number coming in greater than previous number, then
        // line-by-line increased.
        if self.prev_values[2] < val {
            self.increase1_count += 1;
        }

        // If number entering window greater than number coming out, then
        // sliding-window depth increased.
        if self.prev_values[0] < val {
            self.increase3_count += 1;
        }

        // Shift the sliding window.
        self.prev_values[0] = self.prev_values[1];
        self.prev_values[1] = self.prev_values[2];
        self.prev_values[2] = val;
    }

    fn report(&self) {
        println!("Number of increases (line-by-line):   {}", self.increase1_count);
        println!("Number of increases (sliding window): {}", self.increase3_count);
    }
}

fn read_value() -> Option<i32> {
    // Read line from stdin.
    let mut line = String::new();
    return match std::io::stdin().read_line(&mut line) {
        Err(e) => panic!("Failed to read line: {}", e),
        Ok(0) => None,
        Ok(_) =>
            // Parse int from line
            match line.trim().parse::<i32>() {
                Err(_) => panic!("Could not parse i32 from line: {}", line),
                Ok(val) => Some(val)
            }
    };
}

fn main() {
    // Initialize the sliding window.
    let mut sweeper = SonarSweep::new(
        read_value().expect("Could not read 1st value"),
        read_value().expect("Could not read 2nd value"),
        read_value().expect("Could not read 3rd value")
    );

    // Sweep values from stdin.
    while let Some(value) = read_value() {
        sweeper.sweep(value);
    }

    // Print results
    sweeper.report();
}
