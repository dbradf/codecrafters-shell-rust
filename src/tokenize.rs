#[derive(Debug)]
pub struct TokenizedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub append_stdout: bool,
    pub stderr: Option<String>,
    pub append_stderr: bool,
}

enum State {
    Default,
    InDoubleQuotes,
    InSingleQuotes,
    RedirectStdout,
    RedirectStderr,
}

pub fn tokenize_input(input: &str) -> TokenizedCommand {
    let mut state = State::Default;
    let mut tokens = vec![];
    let mut current_token = String::new();
    let mut last_character = None;
    let mut should_escape = false;
    let mut stdout = None;
    let mut append_stdout = false;
    let mut stderr = None;
    let mut append_stderr = false;

    let letters: Vec<char> = input.chars().collect();
    for i in 0..letters.len() {
        let ch = letters[i];

        match state {
            State::Default | State::RedirectStdout | State::RedirectStderr => {
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
                            match state {
                                State::RedirectStdout => {
                                    stdout = Some(current_token.clone());
                                    state = State::Default;
                                }
                                State::RedirectStderr => {
                                    stderr = Some(current_token.clone());
                                    state = State::Default;
                                }
                                _ => match current_token.as_str() {
                                    "1>" | ">" => {
                                        state = State::RedirectStdout;
                                    }
                                    "1>>" | ">>" => {
                                        state = State::RedirectStdout;
                                        append_stdout = true;
                                    }
                                    "2>" => {
                                        state = State::RedirectStderr;
                                    }
                                    "2>>" => {
                                        state = State::RedirectStderr;
                                        append_stderr = true;
                                    }
                                    _ => {
                                        tokens.push(current_token.clone());
                                    }
                                },
                            }
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
        match state {
            State::RedirectStdout => {
                stdout = Some(current_token);
            }
            State::RedirectStderr => {
                stderr = Some(current_token);
            }
            _ => {
                tokens.push(current_token);
            }
        }
    }

    TokenizedCommand {
        command: tokens.first().unwrap().clone(),
        args: tokens[1..].to_vec(),
        stdout,
        append_stdout,
        stderr,
        append_stderr,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("echo hello     world", vec!["hello", "world"])]
    #[case("echo 'hello    world'", vec!["hello    world"])]
    #[case("echo hello''world", vec!["helloworld"])]
    #[case("echo 'hello''world'", vec!["helloworld"])]
    fn test_single_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input).args;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo \"hello    world\"", vec!["hello    world"])]
    #[case("echo \"hello\"\"world\"", vec!["helloworld"])]
    #[case("echo \"shell's test\"", vec!["shell's test"])]
    fn test_double_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input).args;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo three\\ \\ \\ spaces", vec!["three   spaces"])]
    #[case("echo before\\    after", vec!["before ", "after"])]
    #[case("echo test\\nexample", vec!["testnexample"])]
    #[case("echo hello\\\\world", vec!["hello\\world"])]
    #[case("echo \\'hello\\'", vec!["'hello'"])]
    fn test_backslash_escaping(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input).args;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("echo \"A \\\\ escapes itself\"", vec!["A \\ escapes itself"])]
    #[case("echo \"A \\\" inside double quotes\"", vec!["A \" inside double quotes"])]
    #[case("echo \"hello\\\"insidequotes\"script\\\"", vec!["hello\"insidequotesscript\""])]
    #[case("cat \"/tmp/cow/'f  \\34'\"", vec!["/tmp/cow/'f  \\34'"])]
    fn test_double_quote_escapes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input).args;
        assert_eq!(result, expected);
    }
}
