use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::line_buffer::{DeleteListener, Direction, LineBuffer};
use rustyline::validate::Validator;
use rustyline::{Changeset, Context, Helper, RepeatCount, Word};

type AnyError = Box<dyn std::error::Error>;

const PROMPT: &'static str = ">> ";

struct IngestionHelper {}

impl IngestionHelper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_names_to_complete() -> Vec<String> {
        let names = vec!["asdf", "qwer"];
        names.iter().map(|s| s.to_string()).collect()
    }
}
impl Helper for IngestionHelper {}
impl Highlighter for IngestionHelper {}
impl Validator for IngestionHelper {}
impl Hinter for IngestionHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Completer for IngestionHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let names = Self::get_names_to_complete();
        let request = &line[..pos];
        let prefix_matches  = names
            .into_iter()
            .filter(|candidate| candidate.starts_with(request))
            .collect::<Vec<_>>();
        Ok((prefix_matches.len(), prefix_matches))
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let deleted = line.delete_prev_word::<NoListener>(Word::Big, RepeatCount::default(), &mut NoListener {});
        line.replace(line.pos()..line.pos(), elected, cl);
    }
}

pub struct NoListener;

impl DeleteListener for NoListener {
    fn delete(&mut self, _idx: usize, _string: &str, _dir: Direction) {}
}

fn main() -> Result<(), AnyError> {
    let mut rl = rustyline::Editor::<IngestionHelper, DefaultHistory>::new()?;

    rl.set_helper(Some(IngestionHelper::new()));
    loop {
        let readline = rl.readline(PROMPT);

        match readline {
            Ok(line) => println!("Line: {:?}", line),
            Err(_) => {
                println!("No input");
                break;
            }
        }
    }
    Ok(())
}
