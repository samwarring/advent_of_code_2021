mod tree;

struct DiagnosticReader {
    occurences_of_0: [i32; 12],
    occurences_of_1: [i32; 12],
    gamma: i32,
    epsilon: i32,
    power_consumption: i32,
    bit_tree: tree::Tree,
    o2_generator_rating: i32,
    co2_scrubber_rating: i32,
    life_support_rating: i32
}

impl DiagnosticReader {
    fn new() -> Self {
        DiagnosticReader{
            occurences_of_0: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            occurences_of_1: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            gamma: 0,
            epsilon: 0,
            power_consumption: 0,
            bit_tree: tree::Tree::new(),
            o2_generator_rating: 0,
            co2_scrubber_rating: 0,
            life_support_rating: 0
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

        // Update binary tree with the current line.
        self.bit_tree.insert(line.chars());
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
            else if self.occurences_of_1[bit_pos] > self.occurences_of_0[bit_pos] {
                gamma_bits.push('1');
                epsilon_bits.push('0');
            }
            else {
                panic!("Bit pos {} has equal number of 0s and 1s", bit_pos);
            }
        }

        // Convert binary strings into integers (e.g. "1110" = 14)
        self.gamma = i32::from_str_radix(gamma_bits.as_str(), 2).expect("Gamma isn't binary");
        self.epsilon = i32::from_str_radix(epsilon_bits.as_str(), 2).expect("Epsilon isn't binary");

        // Finally, compute power consumption from the product of gamma and epsilon
        self.power_consumption = self.gamma * self.epsilon;
    }

    fn compute_life_support_rating(&mut self) {
        self.compute_o2_generator_rating();
        self.compute_co2_scrubber_rating();
        self.life_support_rating = self.o2_generator_rating * self.co2_scrubber_rating;
    }

    fn compute_o2_generator_rating(&mut self) {
        let mut bits = String::new();
        let mut node: &tree::Tree = &self.bit_tree;
        loop {
            // Get size of 0-subtree
            let count0 = match &node.bit0 {
                None => 0,
                Some(tree0) => tree0.size
            };
            // Get size of 1-subtree
            let count1 = match &node.bit1 {
                None => 0,
                Some(tree1) => tree1.size
            };
            if count0 == 0 && count1 == 0 {
                // This node is a leaf. Stop iterating.
                break;
            }
            // Follow the subtree with greater size (or subtree 1 if same size)
            if count1 >= count0 {
                bits.push('1');
                match &node.bit1 {
                    None => panic!("Subtree 1 magically disappeared"),
                    Some(tree1) => node = &tree1
                };
            }
            else {
                bits.push('0');
                match &node.bit0 {
                    None => panic!("Subtree 0 magically disappeared"),
                    Some(tree0) => node = &tree0
                };
            }
        }
        println!("Computed O2 Generator rating = {}", bits);
        self.o2_generator_rating = i32::from_str_radix(bits.as_str(), 2).expect("O2 generator rating somehow not binary");
    }

    fn compute_co2_scrubber_rating(&mut self) {
        let mut bits = String::new();
        let mut node: &tree::Tree = &self.bit_tree;
        loop {
            // Get size of 0-subtree
            let count0 = match &node.bit0 {
                None => 0,
                Some(tree0) => tree0.size
            };
            // Get size of 1-subtree
            let count1 = match &node.bit1 {
                None => 0,
                Some(tree1) => tree1.size
            };
            
            // Selecting the subtree with smaller size is tricker than selecting the subtree
            // with larger size, because this can lead to us selecting an empty subtree (size=0).
            // To avoid this, if a subtree is empty, let's always choose the opposite subtree.

            let bit: char;
            if count0 == 0 && count1 == 0 {
                // This node is a leaf. Stop iterating.
                break;
            }
            else if count0 == 0 {
                // Subtree 0 is empty. Must follow subtree 1
                bit = '1';
            }
            else if count1 == 0 {
                // Subtree 1 is empty. Must follow subtree 0
                bit = '0';
            }
            else if count0 <= count1 {
                // Neither is empty. Subtree 0 has lesser (or equal) size
                bit = '0';
            }
            else {
                // Neither is empty. Subtree 1 has lesser size
                bit = '1';
            }
            
            // Follow the selected subtree.
            bits.push(bit);
            if bit == '0' {
                match &node.bit0 {
                    None => panic!("Subtree 0 magically disappeared"),
                    Some(tree0) => node = &tree0
                };
            }
            else {
                match &node.bit1 {
                    None => panic!("Subtree 1 magically disappeared"),
                    Some(tree1) => node = &tree1
                };
            }
        }
        println!("Computed CO2 Scrubber rating = {}", bits);
        self.co2_scrubber_rating = i32::from_str_radix(bits.as_str(), 2).expect("CO2 scrubber rating somehow not binary");
    }

    fn report(&self) {
        println!("Gamma: {}", self.gamma);
        println!("Epsilon: {}", self.epsilon);
        println!("Power consumption: {}", self.power_consumption);
        println!("O2 generator rating: {}", self.o2_generator_rating);
        println!("CO2 scrubber rating: {}", self.co2_scrubber_rating);
        println!("Life support rating: {}", self.life_support_rating);
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
    diag.compute_life_support_rating();
    diag.report();
}
