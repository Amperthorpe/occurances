use clap::{Arg, Command};
use crate::shell;

pub fn run_cli() {
    let matches = Command::new("occurances_app")
        .version("0.1.0")
        .about("A simple occurrence tracking application")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("shell")
                .about("Run interactive shell mode")
        )
        .subcommand(
            Command::new("tui")
                .about("Run terminal user interface mode")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("shell", _)) => {
            shell::run_shell();
        }
        Some(("tui", _)) => {
            if let Err(e) = crate::tui::tui_main() {
                eprintln!("TUI error: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            // Default to shell mode if no subcommand is provided
            shell::run_shell();
        }
    }
}
