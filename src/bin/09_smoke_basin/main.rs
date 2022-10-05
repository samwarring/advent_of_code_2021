// lazysort lets us sort the first N elements of a vector.
extern crate lazysort;
use lazysort::SortedBy;

struct Grid<T> {
    rows: usize,
    cols: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    fn new() -> Self {
        Grid {
            rows: 0,
            cols: 0,
            values: Vec::new(),
        }
    }

    fn at(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        &self.values[(row * self.cols) + col]
    }
}

type HeightMap = Grid<i8>;

impl HeightMap {
    fn new_from_stdin() -> Self {
        let mut hm = HeightMap::new();

        let mut line = String::new();
        while let Ok(n) = std::io::stdin().read_line(&mut line) {
            if n == 0 {
                break;
            }
            else {
                let trimmed = line.trim();
                if trimmed.len() != hm.cols && hm.cols > 0 {
                    panic!(
                        "Encountered row with {} cols, but previous rows had {} cols.",
                        trimmed.len(), hm.cols);
                }
                else {
                    hm.cols = trimmed.len();
                    hm.values.reserve(hm.cols);
                    trimmed.chars().for_each(|c| {
                        hm.values.push(c.to_digit(10).unwrap() as i8);
                    });
                    hm.rows += 1;
                }
            }
            line.clear();
        }

        hm
    }

    fn low_points(&self) -> Vec<(usize, usize, i8)> {
        let mut res: Vec<(usize, usize, i8)> = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cur = self.at(row, col);

                // Compare to value on the left
                if 0 < col && self.at(row, col - 1) <= cur {
                    continue;
                }
                // Compare to value on the right
                if (col + 1) < self.cols && self.at(row, col + 1) <= cur {
                    continue;
                }
                // Compare to value above
                if 0 < row && self.at(row - 1, col) <= cur {
                    continue;
                }
                // Compare to value below
                if (row + 1) < self.rows && self.at(row + 1, col) <= cur {
                    continue;
                }

                // Current value less than all surrounding values. Add to result.
                res.push((row, col, *cur));
            }
        }

        res
    }

    // Returns a vector of basin sizes.
    fn basins(&self) -> Vec<usize> {
        // Each cell, if it belongs to a basin, is assigned a basin id.
        // Cells with the same basin id belong to the same basin.
        
        // Value at position i is size of basin i
        let mut basin_sizes: Vec<usize> = Vec::new();

        // This holds the basin ids assigned to each cell in the previous row.
        // The first row has no "previous row", effectively the same
        // as a previous row where no cell was assigned a basin id.
        let mut basin_ids: Vec<Option<usize>> = vec![None; self.cols];

        // Visit each cell in row-major order.
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = self.at(row, col);

                let left_basin_id = if col == 0 { None } else { basin_ids[col - 1] };
                let up_basin_id = basin_ids[col];

                if *cell < 9 {
                    // This cell belongs to a basin.

                    match (left_basin_id, up_basin_id) {
                        (None, None) => {
                            // Neither cell to the left nor cell above belongs to a basin.
                            // Create a new basin and assign its id to this cell.
                            let new_basin_id = basin_sizes.len();
                            basin_sizes.push(1);
                            basin_ids[col] = Some(new_basin_id);
                        },

                        (Some(id_left), None) => {
                            // Only cell to the left belongs to a basin. This cell belongs
                            // to the same basin - which now has 1 more cell in it.
                            basin_sizes[id_left] += 1;
                            basin_ids[col] = Some(id_left);
                        },

                        (None, Some(id_up)) => {
                            // Only cell above belongs to a basin. This cell belongs to the
                            // same basin - which now has one more cell in it.
                            basin_sizes[id_up] += 1;
                            basin_ids[col] = Some(id_up);
                        },

                        (Some(id_left), Some(id_up)) => {
                            // The trickiest case! The cell to the left belongs to a basin,
                            // AND the cell above belongs to a basin - but they might have
                            // different ids! In that case, we need to "merge" the basins
                            // together.
                            if id_left != id_up {
                                // Use the left id for the "merged" basin. We do this by
                                // "stealing" the size of the upper basin and adding it
                                // to the size of the left basin.
                                basin_sizes[id_left] += basin_sizes[id_up];
                                basin_sizes[id_up] = 0;
                            }
                            // In either case, we increment the size of the basin on the left.
                            basin_sizes[id_left] += 1;
                            basin_ids[col] = Some(id_left);
                        }
                    };
                }
                else {
                    // This cell does not belong to a basin.
                    basin_ids[col] = None;
                }
            }
        }

        basin_sizes
    }
}

fn main() {
    let hm = HeightMap::new_from_stdin();

    // Part 1. Answer = 550
    let low_point_risk_sum: i32 = hm.low_points()
        .iter()
        .map(|lp|{ (lp.2 + 1) as i32 })
        .sum();
    println!("Sum of low points: {}", low_point_risk_sum);

    // Part 2. Answer = 1100682
    let p: usize = hm.basins()
        .into_iter()
        .filter(|&s|{ s > 0 })
        .sorted_by(|a, b|{ b.cmp(a) }) // reverse the comparison to sort in descending order.
        .take(3)
        .product();
    println!("Product of top 3 basin sizes: {}", p);
}