pub fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
