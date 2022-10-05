/// Read a line from standard input. If EOF, return None.
/// The returned line is not trimmed. Panics if there is
/// any error reading the line.
pub fn stdin_read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Err(e) => panic!("Error reading line from stdin: {}", e),
        Ok(0) => None,
        Ok(_) => Some(line)
    }
}