

use reedline::{default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, KeyCode, KeyModifiers, Reedline, ReedlineEvent, ReedlineMenu, Signal, DefaultPrompt, Completer, Suggestion, Span, StyledText, DefaultPromptSegment};
use nu_ansi_term::{Color, Style};

struct IngestionCompleter;

impl Completer for IngestionCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let line_before_cursor = &line[0..pos];
        let words = get_words(line_before_cursor);
        let candidates = if words.len() == 1 {
            complete_buckets(words.last().unwrap())
        } else if words.len() == 2 {
            complete_date(words.last().unwrap())
        } else {
            complete_stations(words.last().unwrap())
        };
        return candidates_to_suggestions(candidates, words.last().unwrap(), pos)
    }
}

fn get_words(line_before_cursor: &str) -> Vec<String> {
    let mut words = line_before_cursor.split_ascii_whitespace().collect::<Vec<_>>();
    if let Some(char_before_cursor) = line_before_cursor.chars().last() {
        if char_before_cursor == ' ' {
            words.push("");
        }
    }
    if words.is_empty() {
        words.push("");
    }
    words.into_iter().map(|s|s.to_string()).collect()
}

fn find_substr_filter(word: &str, candidates: Vec<&str>) -> Vec<String> {
    candidates.into_iter().filter(|candidate| candidate.to_lowercase().contains(&word.to_lowercase())).map(|s|s.to_string()).collect()
}

fn complete_buckets(word: &str) -> Vec<String> {
    find_substr_filter(word, vec!["am-ingestion-custom-http", "am-ingestion-provider", "am-ingestion-custom-radio"])

}
fn complete_date(word: &str) -> Vec<String> {
    find_substr_filter(word, vec!["2023-07-13"])
}
fn complete_stations(word: &str) -> Vec<String> {
    find_substr_filter(word, vec!["BBC1", "BBC2", "BBC3", "radio-mansfield", "radio-hits-fm", "radio-kiss-fm",
        "BBC-CAMBRIDGE", "BCC-LONDON", "radio-heart-east", "radio-heart-west", "radio-heart-north",
    "very-long-station-because-it-was-renamed-at-some-point"])
}

fn candidates_to_suggestions(candidates: Vec<String>, word: &str, caret_pos: usize) -> Vec<Suggestion> {
    candidates.into_iter().map(|candidate| {
        Suggestion {
            value: candidate,
            description: None,
            extra: None,
            span: Span { start: caret_pos-word.len(), end: caret_pos },
            append_whitespace: true,
        }
    }).collect()

}

#[cfg(test)]
mod tests {
    use reedline::Span;
    use super::*;

    #[test]
    fn test_candidates_to_suggestions() {
        let candidates = vec!["asdf".to_string()];
        let caret_pos = 2;
        let suggestions = candidates_to_suggestions(candidates,  "df", caret_pos);
        assert_eq!(suggestions, vec![Suggestion {
            value: "asdf".to_string(),
            description: None,
            extra: None,
            span: Span { start: 0, end: caret_pos },
            append_whitespace: true,
        }])
    }
    
    #[test]
    fn test_get_words_empty() {
        assert_eq!(get_words(""), vec![""]);
    }
    #[test]
    fn test_get_words_first_word() {
        assert_eq!(get_words("word"), vec!["word"]);
    }
    #[test]
    fn test_get_words_second_word_empty() {
        assert_eq!(get_words("word "), vec!["word", ""]);
    }
    #[test]
    fn test_get_words_second_word() {
        assert_eq!(get_words("word another"), vec!["word", "another"]);
    }
}

fn main() {
    let completer = Box::new(IngestionCompleter);
    // Use the interactive menu to select options from the completer
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu")
        .with_text_style(Style::new().fg(Color::LightGreen).on(Color::Black))
        .with_selected_text_style(Style::new().fg(Color::Black).on(Color::LightGreen))
        );
    // Set up the required keybindings
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode);

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Empty,
        right_prompt: DefaultPromptSegment::Empty,
    };

    println!("You can write: '<bucket> <date> <station> [<more_stations> ...]'");
    println!("Press TAB at any time to get suggestions, navigate the suggestions with TAB or arrows, accept with enter.");
    println!("You can write substrings to filter suggestions, e.g., 'custom<TAB>', or <TAB>custom");
    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("We processed: {}", buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
