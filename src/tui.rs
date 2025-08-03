use color_eyre::eyre::{Ok, Result};
use ratatui::{DefaultTerminal, Terminal, crossterm::terminal};

pub fn tui_main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    Ok(())
}
