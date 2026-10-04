use std::{collections::HashMap, rc::Rc, sync::Mutex};

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

#[derive(Debug, Clone)]
enum TokenizeState {
    Default,
    InDoubleQuotes,
    InSingleQuotes,
    RedirectStdout,
    RedirectStderr,
}

#[derive(Debug)]
struct State {
    tokenize_state: TokenizeState,
    previous_state: TokenizeState,
    tokens: Vec<String>,
    current_token: String,
    last_char: Option<char>,
    should_escape: bool,
    stdout: Option<String>,
    append_stdout: bool,
    stderr: Option<String>,
    append_stderr: bool,
    variable: Option<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tokenize_state: TokenizeState::Default,
            previous_state: TokenizeState::Default,
            tokens: vec![],
            current_token: String::new(),
            last_char: None,
            should_escape: false,
            stdout: None,
            append_stdout: false,
            stderr: None,
            append_stderr: false,
            variable: None,
        }
    }

    pub fn push_state(&mut self, token_state: TokenizeState) {
        self.previous_state = self.tokenize_state.clone();
        self.tokenize_state = token_state;
    }

    pub fn pop_state(&mut self) {
        self.tokenize_state = self.previous_state.clone();
        self.previous_state = TokenizeState::Default;
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

pub fn tokenize_input(
    input: &str,
    symbol_table: Rc<Mutex<HashMap<String, String>>>,
) -> TokenizeResult {
    let mut commands: Vec<TokenizedCommand> = vec![];
    let mut state = State::new();

    let letters: Vec<char> = input.chars().collect();
    for i in 0..letters.len() {
        let ch = letters[i];

        // is this the end of a variable
        if let Some(var) = &state.variable {
            if ch.is_alphanumeric() || ch == '_' {
                state.variable = Some(format!("{var}{ch}"));
                continue;
            }

            let symbol_table = symbol_table.lock().unwrap();
            if let Some(value) = symbol_table.get(&var[1..]) {
                state.current_token.push_str(&value);
            } else {
                state.current_token.push_str(var);
            }
            state.variable = None;
        }

        if ch == '$' {
            state.variable = Some("$".to_string());
            continue;
        }

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
                        state.push_state(TokenizeState::InSingleQuotes);
                    }
                    '\"' => {
                        if state.handle_consecutive_quotes(letters.get(i + 1), '\"') {
                            continue;
                        }
                        state.push_state(TokenizeState::InDoubleQuotes);
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
                    state.pop_state();
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
                        state.pop_state();
                    }
                    _ => {
                        state.current_token.push(ch);
                    }
                }
            }
        }
    }

    if let Some(var) = &state.variable {
        let symbol_table = symbol_table.lock().unwrap();
        if let Some(value) = symbol_table.get(&var[1..]) {
            state.current_token.push_str(&value);
        } else {
            state.current_token.push_str(var);
        }
        state.variable = None;
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
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input(input, vars);
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
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input(input, vars);
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
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input(input, vars);
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
    #[case("cat /tmp/ant/\"number 26\"", vec!["/tmp/ant/number 26"])]
    fn test_double_quote_escapes(#[case] input: &str, #[case] expected: Vec<&str>) {
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input(input, vars);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    #[case("echo -n \"raspberry strawberry.\" > \"/tmp/ant/number 62\"", vec!["-n", "raspberry strawberry."], "/tmp/ant/number 62")]
    fn test_redirection(
        #[case] input: &str,
        #[case] expected: Vec<&str>,
        #[case] expected_stdout: &str,
    ) {
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input(input, vars);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.args, expected);
                assert_eq!(command.stdout, Some(expected_stdout.to_string()))
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    fn test_pipelines() {
        let vars = Rc::new(Mutex::new(HashMap::new()));
        let result = tokenize_input("cat /tmp/foo/file | wc", vars);
        match result {
            TokenizeResult::Pipeline(commands) => {
                assert_eq!(commands.len(), 2);
                assert_eq!(commands[0].command, "cat");
                assert_eq!(commands[1].command, "wc");
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }

    #[rstest]
    fn test_variables() {
        let vars = Rc::new(Mutex::new(HashMap::new()));
        {
            let mut vars = vars.lock().unwrap();
            vars.insert("hello".to_string(), "world".to_string());
        }
        let result = tokenize_input("echo $hello", vars);
        match result {
            TokenizeResult::SingleCommand(command) => {
                assert_eq!(command.command, "echo".to_string());
                assert_eq!(command.args, vec!["world".to_string()]);
            }
            _ => panic!("unexpect result {:?}", result),
        }
    }
}
