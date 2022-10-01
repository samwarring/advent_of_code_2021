type Num = i32;   // A value on the board
type Row = usize; // A row index
type Col = usize; // A col index
type Score = i32; // A final score

const BOARD_ROWS: Row = 5; // Number of rows for a board.
const BOARD_COLS: Col = 5; // Number of cols for a board.

fn parse_num(int_str: &str) -> Num {
    Num::from_str_radix(int_str, 10).unwrap_or_else(|_|{
        panic!("Failed to parse Num from {}", int_str)
    })
}

struct Board {
    // A list of every value on the board, paired with its position. This
    // will be sorted by the value.
    layout: Vec<(Num, Row, Col)>,

    // A list of values that have not yet been called. This will be sorted.
    remaining: Vec<Num>,

    // Number of values called for each row (e.g. called_in_row[2] = 3 means
    // that row 2 has 3 numbers that have been called).
    called_in_row: [usize; BOARD_ROWS],

    // Ditto, but for the columns.
    called_in_col: [usize; BOARD_COLS]
}

impl Board {
    // Adds row data to a new board.
    fn add_row(&mut self, row: Row, row_vals: Vec<Num>) {
        assert!(row < BOARD_ROWS);
        assert!(row_vals.len() == BOARD_COLS);
        for (col, val) in row_vals.iter().enumerate() {
            self.layout.push((*val, row, col));
            self.remaining.push(*val);
        }
    }

    // Finalizes the new board by sorting the values for easy indexing.
    fn sort_values(&mut self) {
        self.layout.sort_unstable_by_key(|nrc|{ nrc.0 });
        self.remaining.sort_unstable();
    }

    // Read a full board from stdin. If nothing to parse, return None.
    // If parsing stops mid-board, panic!
    fn new_from_stdin() -> Option<Self> {
        
        // Fill in this board object
        let mut board = Board{
            layout: Vec::new(),
            remaining: Vec::new(),
            called_in_row: [0; BOARD_ROWS],
            called_in_col: [0; BOARD_COLS]
        };

        // Read lines from stdin
        let mut row: Row = 0;
        while row < BOARD_ROWS {
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Err(_) | Ok(0) => return None,
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        if row == 0 {
                            continue;
                        }
                        else {
                            panic!("Encountered incomplete board from stdin");
                        }
                    }
                    // Parse line and add it as a new row to the board.
                    let row_vals: Vec<Num> = trimmed.split_ascii_whitespace().map(parse_num).collect();
                    board.add_row(row, row_vals);
                    row += 1;
                }
            }
        }
        board.sort_values();
        return Some(board);
    }

    // Mark this number of the board. If it resulted in a BINGO! then
    // return Some(score); otherwise, return None.
    fn on_called_number(&mut self, num: Num) -> Option<Score> {
        // Find the value in the layout.
        if let Ok(index) = self.layout.binary_search_by_key(&num, |nrc|{ nrc.0 }) {
            self.remaining.remove(self.remaining.binary_search(&num).expect("Not found in remaining"));
            let (_, row, col) = self.layout[index];
            self.called_in_row[row] += 1;
            self.called_in_col[col] += 1;
            if self.called_in_row[row] == BOARD_COLS || self.called_in_col[col] == BOARD_ROWS{
                // BINGO!
                return Some(self.score(num));
            }
        }
        None
    }

    fn score(&self, last_called: Num) -> Score {
        let mut s: Score = 0;
        for val in &self.remaining {
            s += val;
        }
        return s * last_called;
    }
}

fn main() {
    // Read numbers that are called.
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Could not read called numbers. EOF");
    let called_numbers: Vec<Num> = line.trim().split(',').map(parse_num).collect();
    println!("Called numbers (parsed): {:?}", called_numbers);

    // Read all the boards.
    let mut board_id: i32 = 0;
    let mut boards: Vec<(i32, Box<Board>)> = Vec::new();
    while let Some(board) = Board::new_from_stdin() {
        boards.push((board_id, Box::new(board)));
        board_id += 1;
    }
    println!("Read {} boards", boards.len());

    // Start calling numbers until a board gets a bingo.
    for num in called_numbers {
        println!("Calling {}", num);
        boards.retain_mut(|item| -> bool {
            let (id, board) = item;
            if let Some(score) = board.on_called_number(num) {
                println!("BINGO! on board {} with score {}", id, score);
                return false;
            }
            return true;
        });
    }
}
