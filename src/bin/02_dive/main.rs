enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

// Reads a command from stdin.
fn read_command() -> Option<Command> {
    let mut line = String::new();
    return match std::io::stdin().read_line(&mut line) {
        Err(e) => panic!("Failed to read line: {}", e),
        Ok(0) => None,
        Ok(_) => Some(parse_command(line.trim()))
    };
}

// Parses command from a string.
fn parse_command(line: &str) -> Command {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
    assert_eq!(tokens.len(), 2, "Command must have 2 tokens");
    let value: i32 = match tokens[1].parse() {
        Err(_) => panic!("Command value is not an integer: {}", tokens[1]),
        Ok(x) => x
    };
    return match tokens[0] {
        "forward" => Command::Forward(value),
        "down" => Command::Down(value),
        "up" => Command::Up(value),
        _ => panic!("Invalid command: {}", tokens[0])
    };
}

// Holds the position/aim of the submarine.
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32
}

impl Submarine {
    fn new() -> Self {
        Submarine{
            horizontal: 0,
            aim: 0,
            depth: 0
        }
    }

    // Updates the position/aim according to the command.
    fn do_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
            },
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x
        };
    }

    // Prints the final results.
    fn report(&self) {
        println!("Horizontal position: {}", self.horizontal);
        println!("Depth: {}", self.depth);
        println!("Product: {}", self.horizontal * self.depth);
    }
}

fn main() {
    let mut sub = Submarine::new();
    while let Some(cmd) = read_command() {
        sub.do_command(&cmd);
    }
    sub.report();
}
