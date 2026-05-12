pub fn get_line_content(text: &str, line: usize) -> Option<&str> {
    text.lines().nth(line)
}
