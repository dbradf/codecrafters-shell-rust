enum State {
    Default,
    InDoubleQuotes,
    InSingleQuotes,
}

pub fn tokenize_input(input: &str) -> Vec<String> {
    let mut state = State::Default;
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut last_character = None;
    let mut should_escape = false;

    let letters: Vec<char> = input.chars().collect();
    for i in 0..letters.len() {
        let ch = letters[i];

        match state {
            State::Default => {
                if should_escape {
                    current_token.push(ch);
                    should_escape = false;
                    continue;
                }

                match ch {
                    '\\' => {
                        should_escape = true;
                    }
                    '\'' => {
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
                    }
                    '\"' => {
                        if let Some(next_ch) = letters.get(i + 1)
                            && *next_ch == '\"'
                        {
                            last_character = Some('\"');
                            continue;
                        }
                        if last_character == Some('\"') {
                            // ignore "".
                            continue;
                        }
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                        state = State::InDoubleQuotes;
                    }
                    ch if ch.is_whitespace() => {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                    _ => {
                        current_token.push(ch);
                    }
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
            State::InDoubleQuotes => {
                if should_escape {
                    should_escape = false;
                    match ch {
                        '\"' | '\\' => {
                            current_token.push(ch);
                            continue;
                        }
                        _ => {
                            current_token.push('\\');
                        }
                    }
                }

                match ch {
                    '\\' => {
                        should_escape = true;
                        continue;
                    }
                    '\"' => {
                        if let Some(next_ch) = letters.get(i + 1)
                            && *next_ch == '\"'
                        {
                            // ignore "".
                            last_character = Some('\"');
                            continue;
                        }
                        if last_character == Some('\"') {
                            // ignore "".
                            last_character = None;
                            continue;
                        }
                        state = State::Default;
                    }
                    _ => {
                        current_token.push(ch);
                    }
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
    fn test_single_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo \"hello    world\"", vec!["echo", "hello    world"])]
    #[case("echo \"hello\"\"world\"", vec!["echo", "helloworld"])]
    #[case("echo \"shell's test\"", vec!["echo", "shell's test"])]
    fn test_double_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo three\\ \\ \\ spaces", vec!["echo", "three   spaces"])]
    #[case("echo before\\    after", vec!["echo", "before ", "after"])]
    #[case("echo test\\nexample", vec!["echo", "testnexample"])]
    #[case("echo hello\\\\world", vec!["echo", "hello\\world"])]
    #[case("echo \\'hello\\'", vec!["echo", "'hello'"])]
    fn test_backslash_escaping(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo \"A \\\\ escapes itself\"", vec!["echo", "A \\ escapes itself"])]
    #[case("echo \"A \\\" inside double quotes\"", vec!["echo", "A \" inside double quotes"])]
    #[case("echo \"hello\\\"insidequotes\"script\\\"", vec!["echo", "hello\"insidequotesscript\""])]
    #[case("cat \"/tmp/cow/'f  \\34'\"", vec!["cat", "/tmp/cow/'f  \\34'"])]
    fn test_double_quote_escapes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        assert_eq!(result, expected);
    }
}
