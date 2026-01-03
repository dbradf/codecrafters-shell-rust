enum State {
    InSingleQuotes,
    Default,
}

pub fn parse_command(input: &str) -> Vec<String> {
    let mut state = State::Default;
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut last_character = None;

    let letters: Vec<char> = input.chars().collect();
    for i in 0..letters.len() {
        let ch = letters[i];
        match state {
            State::Default => {
                if ch.is_whitespace() {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else if ch == '\'' {
                    if let Some(next_ch) = letters.get(i + 1)
                        && *next_ch == '\''
                    {
                        last_character = Some('\'');
                        continue;
                    }
                    if last_character == Some('\'') {
                        // ignore ''.
                        continue;
                    }
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    state = State::InSingleQuotes;
                } else {
                    current_token.push(ch);
                }
            }
            State::InSingleQuotes => {
                if ch == '\'' {
                    if let Some(next_ch) = letters.get(i + 1)
                        && *next_ch == '\''
                    {
                        // ignore ''.
                        last_character = Some('\'');
                        continue;
                    }
                    if last_character == Some('\'') {
                        // ignore ''.
                        last_character = None;
                        continue;
                    }
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    state = State::Default;
                } else {
                    current_token.push(ch);
                }
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("echo hello     world", vec!["echo", "hello", "world"])]
    #[case("echo 'hello    world'", vec!["echo", "hello    world"])]
    #[case("echo hello''world", vec!["echo", "helloworld"])]
    #[case("echo 'hello''world'", vec!["echo", "helloworld"])]
    fn test_empty_single_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = parse_command(input);
        assert_eq!(result, expected);
    }
}
