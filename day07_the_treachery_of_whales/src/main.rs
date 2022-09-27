fn fuel(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|c|{

         let distance = (c - pos).abs();
         (distance * (distance + 1)) / 2

     }).sum()
}

fn main() {
    // Read input
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let crabs: Vec<i32> = line.trim().split(',').map(|s|{ i32::from_str_radix(s, 10).unwrap() }).collect();

    // Compute minimal fuel usage by trying all points between 0 and the max position.
    let max_pos = crabs.iter().max().unwrap();
    let mut min_fuel: Option<i32> = None;
    for i in 0..=*max_pos {
        let cur_fuel = fuel(&crabs, i);
        if let Some(cur_min) = min_fuel {
            if cur_fuel < cur_min {
                min_fuel = Some(cur_fuel);
            }
        }
        else {
            min_fuel = Some(cur_fuel);
        }
    }

    println!("Min fuel = {}", min_fuel.unwrap());
}
