struct DiagnosticReader {
    occurences_of_0: [i32; 12],
    occurences_of_1: [i32; 12],
    gamma: i32,
    epsilon: i32,
    power_consumption: i32
}

impl DiagnosticReader {
    fn new() -> Self {
        DiagnosticReader{
            occurences_of_0: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            occurences_of_1: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            gamma: 0,
            epsilon: 0,
            power_consumption: 0
        }
    }

    // Extract each bit from the input line. Update occurences of 0/1 at each
    // bit position.
    fn read_line(&mut self, line: &str) {
        assert_eq!(line.chars().count(), 12, "Line does not contain 12 bits: {}:::", line);
        for (bit_pos, ch) in line.chars().enumerate() {
            match ch {
                '0' => self.occurences_of_0[bit_pos] += 1,
                '1' => self.occurences_of_1[bit_pos] += 1,
                _ => panic!("Line contains invalid bit: {}", line)
            };
        }
    }

    fn compute_power_consumption(&mut self) {
        // Binary-string representations of gamma and epsilon
        let mut gamma_bits = String::new();
        let mut epsilon_bits = String::new();

        // Determine most common bit from each column (bit position)
        for bit_pos in 0..12 {
            if self.occurences_of_0[bit_pos] > self.occurences_of_1[bit_pos] {
                gamma_bits.push('0');
                epsilon_bits.push('1');
            }
            else {
                gamma_bits.push('1');
                epsilon_bits.push('0');
            }
        }

        // Convert binary strings into integers (e.g. "1110" =  14)
        self.gamma = i32::from_str_radix(gamma_bits.as_str(), 2).expect("Gamma isn't binary");
        self.epsilon = i32::from_str_radix(epsilon_bits.as_str(), 2).expect("Epsilon isn't binary");

        // Finally, compute power consumption from the product of gamma and epsilon
        self.power_consumption = self.gamma * self.epsilon;
    }

    fn report(&self) {
        println!("Gamma: {}", self.gamma);
        println!("Epsilon: {}", self.epsilon);
        println!("Power consumption: {}", self.power_consumption);
    }
}

fn main() {
    let mut diag = DiagnosticReader::new();
    let mut done = false;
    while !done {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Err(e) => panic!("Error reading line: {}", e),
            Ok(0) => done = true,
            Ok(_) => {
                diag.read_line(line.trim())
            }
        };
    }
    diag.compute_power_consumption();
    diag.report();
}
