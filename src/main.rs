
type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError>{
    let mut rl = rustyline::DefaultEditor::new()?;
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => println!("Line: {:?}", line),
        Err(_) => println!("No input"),
    }
    Ok(())
}
