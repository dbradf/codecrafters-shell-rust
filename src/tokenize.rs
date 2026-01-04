#[derive(Debug, Clone)]
pub struct TokenizedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub append_stdout: bool,
    pub stderr: Option<String>,
    pub append_stderr: bool,
}

#[derive(Debug)]
pub enum TokenizeResult {
    SingleCommand(TokenizedCommand),
    Pipeline(Vec<TokenizedCommand>),
}

enum TokenizeState {
    Default,
    InDoubleQuotes,
    InSingleQuotes,
    RedirectStdout,
    RedirectStderr,
}

struct State {
    tokenize_state: TokenizeState,
    tokens: Vec<String>,
    current_token: String,
    last_char: Option<char>,
    should_escape: bool,
    stdout: Option<String>,
    append_stdout: bool,
    stderr: Option<String>,
    append_stderr: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            tokenize_state: TokenizeState::Default,
            tokens: vec![],
            current_token: String::new(),
            last_char: None,
            should_escape: false,
            stdout: None,
            append_stdout: false,
            stderr: None,
            append_stderr: false,
        }
    }

    pub fn promote_current_token(&mut self) {
        if !self.current_token.is_empty() {
            self.tokens.push(self.current_token.clone());
            self.current_token.clear();
        }
    }

    pub fn handle_consecutive_quotes(
        &mut self,
        next_char: Option<&char>,
        quote_type: char,
    ) -> bool {
        if let Some(next_ch) = next_char
            && *next_ch == quote_type
        {
            self.last_char = Some(quote_type);
            return true;
        }

        if self.last_char == Some(quote_type) {
            // ignore ''.
            self.last_char = None;
            return true;
        }

        false
    }

    pub fn finalize(&mut self) -> TokenizedCommand {
        if !self.current_token.is_empty() {
            match self.tokenize_state {
                TokenizeState::RedirectStdout => {
                    self.stdout = Some(self.current_token.clone());
                }
                TokenizeState::RedirectStderr => {
                    self.stderr = Some(self.current_token.clone());
                }
                _ => {
                    self.tokens.push(self.current_token.clone());
                }
            }
        }

        TokenizedCommand {
            command: self.tokens.first().unwrap().clone(),
            args: self.tokens[1..].to_vec(),
            stdout: self.stdout.clone(),
            append_stdout: self.append_stdout,
            stderr: self.stderr.clone(),
            append_stderr: self.append_stderr,
        }
    }
}

pub fn tokenize_input(input: &str) -> TokenizeResult {
    let mut commands: Vec<TokenizedCommand> = vec![];
    let mut state = State::new();

    let letters: Vec<char> = input.chars().collect();
    for i in 0..letters.len() {
        let ch = letters[i];

        match state.tokenize_state {
            TokenizeState::Default
            | TokenizeState::RedirectStdout
            | TokenizeState::RedirectStderr => {
                if state.should_escape {
                    state.current_token.push(ch);
                    state.should_escape = false;
                    continue;
                }

                match ch {
                    '|' => {
                        commands.push(state.finalize());
                        state = State::new();
                    }
                    '\\' => {
                        state.should_escape = true;
                    }
                    '\'' => {
                        if state.handle_consecutive_quotes(letters.get(i + 1), '\'') {
                            continue;
                        }
                        state.promote_current_token();
                        state.tokenize_state = TokenizeState::InSingleQuotes;
                    }
                    '\"' => {
                        if state.handle_consecutive_quotes(letters.get(i + 1), '\"') {
                            continue;
                        }
                        state.promote_current_token();
                        state.tokenize_state = TokenizeState::InDoubleQuotes;
                    }
                    ch if ch.is_whitespace() => {
                        if !state.current_token.is_empty() {
                            match state.tokenize_state {
                                TokenizeState::RedirectStdout => {
                                    state.stdout = Some(state.current_token.clone());
                                    state.tokenize_state = TokenizeState::Default;
                                }
                                TokenizeState::RedirectStderr => {
                                    state.stderr = Some(state.current_token.clone());
                                    state.tokenize_state = TokenizeState::Default;
                                }
                                _ => match state.current_token.as_str() {
                                    "1>" | ">" => {
                                        state.tokenize_state = TokenizeState::RedirectStdout;
                                    }
                                    "1>>" | ">>" => {
                                        state.tokenize_state = TokenizeState::RedirectStdout;
                                        state.append_stdout = true;
                                    }
                                    "2>" => {
                                        state.tokenize_state = TokenizeState::RedirectStderr;
                                    }
                                    "2>>" => {
                                        state.tokenize_state = TokenizeState::RedirectStderr;
                                        state.append_stderr = true;
                                    }
                                    _ => {
                                        state.tokens.push(state.current_token.clone());
                                    }
                                },
                            }
                            state.current_token.clear();
                        }
                    }
                    _ => {
                        state.current_token.push(ch);
                    }
                }
            }
            TokenizeState::InSingleQuotes => {
                if ch == '\'' {
                    if state.handle_consecutive_quotes(letters.get(i + 1), '\'') {
                        continue;
                    }
                    state.promote_current_token();
                    state.tokenize_state = TokenizeState::Default;
                } else {
                    state.current_token.push(ch);
                }
            }
            TokenizeState::InDoubleQuotes => {
                if state.should_escape {
                    state.should_escape = false;
                    match ch {
                        '\"' | '\\' => {
                            state.current_token.push(ch);
                            continue;
                        }
                        _ => {
                            state.current_token.push('\\');
                        }
                    }
                }

                match ch {
                    '\\' => {
                        state.should_escape = true;
                        continue;
                    }
                    '\"' => {
                        if state.handle_consecutive_quotes(letters.get(i + 1), '\"') {
                            continue;
                        }
                        state.tokenize_state = TokenizeState::Default;
                    }
                    _ => {
                        state.current_token.push(ch);
                    }
                }
            }
        }
    }

    commands.push(state.finalize());
    if commands.len() == 1 {
        TokenizeResult::SingleCommand(commands[0].clone())
    } else {
        TokenizeResult::Pipeline(commands)
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
        let result = tokenize_input(input);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    #[case("echo \"hello    world\"", vec!["hello    world"])]
    #[case("echo \"hello\"\"world\"", vec!["helloworld"])]
    #[case("echo \"shell's test\"", vec!["shell's test"])]
    fn test_double_quotes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    #[case("echo three\\ \\ \\ spaces", vec!["three   spaces"])]
    #[case("echo before\\    after", vec!["before ", "after"])]
    #[case("echo test\\nexample", vec!["testnexample"])]
    #[case("echo hello\\\\world", vec!["hello\\world"])]
    #[case("echo \\'hello\\'", vec!["'hello'"])]
    fn test_backslash_escaping(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    #[case("echo \"A \\\\ escapes itself\"", vec!["A \\ escapes itself"])]
    #[case("echo \"A \\\" inside double quotes\"", vec!["A \" inside double quotes"])]
    #[case("echo \"hello\\\"insidequotes\"script\\\"", vec!["hello\"insidequotesscript\""])]
    #[case("cat \"/tmp/cow/'f  \\34'\"", vec!["/tmp/cow/'f  \\34'"])]
    fn test_double_quote_escapes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let result = tokenize_input(input);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    fn test_pipelines() {
        let result = tokenize_input("cat /tmp/foo/file | wc");
        match result {
            TokenizeResult::Pipeline(commands) => {
                assert_eq!(commands.len(), 2);
                assert_eq!(commands[0].command, "cat");
                assert_eq!(commands[1].command, "wc");
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }
}
