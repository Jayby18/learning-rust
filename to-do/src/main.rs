use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal
};

fn main() -> Results<(), io:Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend.new(stdout);
    let mut terminal - Terminal::new(backend)?;
    Ok(())
}