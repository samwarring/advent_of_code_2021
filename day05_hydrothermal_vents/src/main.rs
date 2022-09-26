use std::cmp::min_by;

struct VentMap {
    x_size: usize,
    y_size: usize,
    grid: Vec<i8>,
}

impl VentMap {
    fn new(x_size: usize, y_size: usize) -> Self {
        VentMap {
            x_size: x_size,
            y_size: y_size,
            grid: vec![0; x_size * y_size],
        }
    }

    fn get(&self, x: usize, y:usize) -> i8 {
        assert!(x < self.x_size);
        assert!(y < self.y_size);
        self.grid[self.x_size * y + x]
    }

    fn set(&mut self, x: usize, y: usize) {
        assert!(x < self.x_size);
        assert!(y < self.y_size);
        self.grid[self.x_size * y + x] += 1;
    }

    fn set_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        assert!(x1 < self.x_size);
        assert!(y1 < self.y_size);
        assert!(x2 < self.x_size);
        assert!(y2 < self.y_size);
        
        if x1 == x2 {
            let mut y = std::cmp::min(y1, y2);
            let y_max = std::cmp::max(y1, y2);
            while y <= y_max {
                self.set(x1, y);
                y += 1;
            }
        }
        else if y1 == y2 {
            let mut x = std::cmp::min(x1, x2);
            let x_max = std::cmp::max(x1, x2);
            while x <= x_max {
                self.set(x, y1);
                x += 1;
            }
        }
    }

    fn get_overlapped_count(&self) -> usize {
        self.grid.iter().filter(|n|{ *n > &1 }).count()
    }
}

fn parse(s: &str) -> usize {
    usize::from_str_radix(s, 10).expect("Failed to parse value")
}

fn main() {
    let mut vm = VentMap::new(1000, 1000);
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Err(_) => panic!("Failed to read line"),
            Ok(0)  => break,
            Ok(_)  => {
                let mut split: Vec<usize> = line
                    .split(|ch|{ !char::is_numeric(ch) })
                    .filter(|s|{ !s.is_empty() })
                    .map(parse)
                    .collect();
                let x1 = split[0];
                let y1 = split[1];
                let x2 = split[2];
                let y2 = split[3];
                println!("({},{}) -> ({},{})", x1, y1, x2, y2);
                vm.set_line(x1, y1, x2, y2)
            }
        }
    }
    println!("{} points have overlapping lines", vm.get_overlapped_count());
}
