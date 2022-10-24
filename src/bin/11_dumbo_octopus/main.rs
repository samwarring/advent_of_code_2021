use advent_of_code_2021::stdin_read_line;
use std::collections::BTreeSet;

struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Grid { rows, cols, data }
    }

    fn at(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.cols + col]
    }

    fn size(&self) -> usize {
        self.rows * self.cols
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if 0 < row && 0 < col {
            result.push((row - 1, col - 1));
        }
        if 0 < row {
            result.push((row - 1, col));
        }
        if 0 < row && (col + 1) < self.cols {
            result.push((row - 1, col + 1));
        }
        if 0 < col {
            result.push((row, col - 1));
        }
        if (col + 1) < self.cols {
            result.push((row, col + 1));
        }
        if (row + 1) < self.rows && 0 < col {
            result.push((row + 1, col - 1));
        }
        if (row + 1) < self.rows {
            result.push((row + 1, col));
        }
        if (row + 1) < self.rows && (col + 1) < self.cols {
            result.push((row + 1, col + 1));
        }

        result
    }
}

struct CircularQueue<T> {
    data: Vec<T>,
    front: usize,
    size: usize,
}

impl<T: Copy> CircularQueue<T> {
    fn new(capacity: usize) -> Self {
        let mut data: Vec<T> = Vec::new();
        data.reserve(capacity);
        CircularQueue {
            data,
            front: 0,
            size: 0,
        }
    }

    fn push(&mut self, x: T) {
        assert!(self.size < self.data.capacity());
        if self.data.len() == self.data.capacity() {
            let back = if self.front == 0 {
                self.data.capacity() - 1
            } else {
                self.front - 1
            };
            self.data[back] = x;
        } else {
            self.data.push(x);
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let result = Some(self.data[self.front]);
        self.front += 1;
        self.size -= 1;
        result
    }

    fn size(&self) -> usize {
        self.size
    }
}

type EnergyLevel = i32;
type OctopusGrid = Grid<EnergyLevel>;

// Simulate one step. Return number of octopusses that flashed.
fn step(grid: &mut OctopusGrid) -> usize {
    let mut flashes: usize = 0;

    // Increase all energy levels by 1.
    grid.data.iter_mut().for_each(|v| *v += 1);

    // flash_queue holds positions that need to be flashed.
    // flash_set holds positions that have already flashed (should not be re-queued).
    let mut flash_queue: CircularQueue<(usize, usize)> = CircularQueue::new(grid.size());
    let mut flash_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if *grid.at(row, col) > 9 {
                flash_queue.push((row, col));
                flash_set.insert((row, col));
            }
        }
    }

    // Perform the flashes until no more octopusses left to flash
    while let Some(flash_pos) = flash_queue.pop() {
        flashes += 1;
        let (row, col) = flash_pos;

        // For each neighbor...
        grid.neighbors(row, col)
            .into_iter()
            .filter(|rc| {
                let n = grid.at_mut(rc.0, rc.1);
                *n += 1; // increment energy level by 1
                *n > 9 // if new energy level > 9...
            })
            .for_each(|rc| {
                // If not yet flashed this step, add neighbor position to flash queue.
                if flash_set.insert(rc) {
                    flash_queue.push(rc);
                }
            });
    }

    // Octopusses that flashed this step have energy levels return to 0.
    flash_set.into_iter().for_each(|rc| {
        *grid.at_mut(rc.0, rc.1) = 0;
    });

    // Return total number of flashes.
    flashes
}

fn main() {
    // Read input from stdin into a grid struct
    let mut input: Vec<EnergyLevel> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    while let Some(line) = stdin_read_line() {
        let row: Vec<i32> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        assert!(cols == 0 || row.len() == cols);
        cols = row.len();
        input.extend(row.into_iter());
        rows += 1;
    }
    let mut grid: OctopusGrid = Grid::new(rows, cols, input);

    // Simulate steps
    let mut flashes: usize = 0;
    let mut i = 0;
    loop {
        // Part 1 answer: 1681
        i += 1;
        let step_flashes = step(&mut grid);
        flashes += step_flashes;
        println!("Step {}: {} flashes, {} total", i, step_flashes, flashes);

        // Part 2 answer: 276
        if grid.data.iter().sum::<EnergyLevel>() == 0 {
            println!("All octopusses flashed in the last step!");
            break;
        }
    }
}
