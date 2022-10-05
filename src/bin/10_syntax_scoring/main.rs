use advent_of_code_2021::stdin_read_line;
use lazysort::Sorted;

fn score_syntax_error(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("score(SyntaxError({}))", c)
    }
}

fn score_incomplete(tail: &str) -> i64 {
    let mut total = 0;
    for c in tail.chars() {
        total *= 5;
        total += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("score(Incomplete({}))", c)
        };
    }
    total
}

fn opposite(c: char) -> char {
    match c {
        // Open characters
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        // Close characters
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("opposite({})", c)
    }
}

#[derive(Debug)]
enum ParseResult {
    Ok,
    Incomplete(String),
    SyntaxError(char),
}

fn parse(line: &str) -> ParseResult {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '('|'['|'{'|'<' => stack.push(c),
            ')'|']'|'}'|'>' => {
                if let Some(top) = stack.pop() {
                    if opposite(c) != top {
                        return ParseResult::SyntaxError(c);
                    }
                }
                else {
                    return ParseResult::SyntaxError(c);
                }
            },
            _ => panic!("Invalid character on line: '{}'", c)
        };
    }
    if stack.is_empty() {
        ParseResult::Ok
    }
    else {
        let mut tail = String::new();
        while let Some(top) = stack.pop() {
            tail.push(opposite(top));
        }
        ParseResult::Incomplete(tail)
    }
}

fn main() {
    let mut error_score = 0;
    let mut incomplete_scores: Vec<i64> = Vec::new();

    while let Some(line) = stdin_read_line() {
        let trimmed = line.trim();
        let result = parse(trimmed);
        println!("{:<110}{:?}", trimmed, result);
        
        if let ParseResult::SyntaxError(c) = result {
            error_score += score_syntax_error(c);
        } else if let ParseResult::Incomplete(tail) = result {
            incomplete_scores.push(score_incomplete(tail.as_str()));
        }
    }

    // Part 1 answer: 168417
    println!("Total error score: {}", error_score);

    // Part 2 answer: 2802519786
    let median_pos = incomplete_scores.len() / 2 + 1;
    let median_score = incomplete_scores
        .into_iter()
        .sorted()
        .take(median_pos)
        .last()
        .unwrap();
    println!("Winning incomplete score: {}", median_score);
}