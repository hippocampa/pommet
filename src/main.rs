use core::App;
use crossterm::style::{Color, Stylize};
use std::io;

mod core;
mod plugins;
mod tui;
fn main() -> io::Result<()> {
    let mut app = App::new();

    println!(
        "{}",
        "Checking plugin installation status..."
            .bold()
            .with(Color::Cyan)
    );
    if !app.all_plugin_installed() {
        println!(
            "{}",
            "Some plugins are not installed. Installing now...".with(Color::Yellow)
        );
        app.install_all_plugins();
        println!(
            "{}",
            "All plugins have been installed successfully."
                .bold()
                .green()
        );

        println!(
            "\n{}",
            "Press Enter to continue to the application..."
                .italic()
                .with(Color::Yellow)
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
    }
    // } else {
    //     println!("{}", "All plugins are already installed.".bold().green());
    // }

    // println!("Launching TUI application...");
    let mut terminal = ratatui::init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
