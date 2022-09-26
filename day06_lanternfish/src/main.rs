struct FishCounter {
    timers: [usize; 9] // at pos i = number of fish with timer = i.
}

impl FishCounter {
    fn new() -> Self {
        FishCounter { timers: [0; 9] }
    }

    fn add_initial_fish(&mut self, timer: usize) {
        self.timers[timer] += 1;
    }

    fn elapse_one_day(&mut self) -> usize {
        self.timers.rotate_left(1);       // All fish decrease their timers by 1. Fish at 0 give birth!
        self.timers[6] += self.timers[8]; // Fish that gave birth have 6 days remaining.
        self.timers[8]                    // Return number of new fish!
    }

    fn get_total(&self) -> usize {
        self.timers.iter().sum()
    }
}

// Day 80: 49056 new fish, 388739 in total
// Day 256: 158720888232 new fish, 1741362314973 in total
fn main() {
    // Read input
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read input");
    let input: Vec<usize> = line.trim()
        .split(',')
        .map(|x|{ usize::from_str_radix(x, 10).expect("Failed to parse integer") })
        .collect();
    
    // Set initial condition
    let mut fc = FishCounter::new();
    for i in input {
        fc.add_initial_fish(i);
    }
    println!("Initial condition: {} fish", fc.get_total());

    // Simulate N days
    for day in 1..=256 {
        let born = fc.elapse_one_day();
        println!("Day {}: {} new fish, {} in total", day, born, fc.get_total());
    }
}
