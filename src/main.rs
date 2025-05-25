use std::error::Error;

use tui::app::App;

mod plugins;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.ensure_installation()?;
    Ok(())

    // let mut terminal = ratatui::init();
    // let app_result = App::new().run(&mut terminal);
    // ratatui::restore();
    // app_result
}
