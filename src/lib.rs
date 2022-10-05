//! Functions commonly used in AOC solutions.

/// Reads an optional line from standard input.
/// 
/// The resulting line, if any, is not trimmed. On EOF, this
/// function returns None.
/// 
/// # Panics
/// 
/// The function panics on any error reading from stdin.
/// 
/// # Examples
/// 
/// ```
/// use advent_of_code_2021::*;
/// while let Some(line) = stdin_read_line() {
///     print!("Read line: {}", line);
/// }
/// println!("End of standard input");
/// ```
pub fn stdin_read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Err(e) => panic!("Error reading line from stdin: {}", e),
        Ok(0) => None,
        Ok(_) => Some(line)
    }
}