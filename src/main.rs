

use reedline::{default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, KeyCode, KeyModifiers, Reedline, ReedlineEvent, ReedlineMenu, Signal, DefaultPrompt};

fn main() {
    let mut commands = vec![
        "test".into(),
        "hello world".into(),
        "hello world reedline".into(),
        "this is the reedline crate".into(),
    ];
    for i in 0..100 {
        commands.push(format!("station{i}"));
    }
    let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
// Use the interactive menu to select options from the completer
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
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

    let prompt = DefaultPrompt::default();

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
