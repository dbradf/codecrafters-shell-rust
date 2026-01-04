use rustyline::completion::{Completer, Pair};

pub struct TermCompleter {
    commands: Vec<String>,
}

impl TermCompleter {
    pub fn new(commands: &[String]) -> Self {
        let mut commands = commands.to_vec();
        commands.sort();
        Self { commands }
    }
}

impl Completer for TermCompleter {
    type Candidate = Pair;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let word_start = line[..pos]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);

        let partial = &line[word_start..pos];

        let matches: Vec<Pair> = self
            .commands
            .iter()
            .filter(|opt| opt.starts_with(partial))
            .map(|opt| Pair {
                display: opt.clone(),
                replacement: format!("{} ", opt),
            })
            .collect();

        Ok((word_start, matches))
    }
}
