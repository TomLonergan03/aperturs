mod app;
pub(crate) mod image;
pub(crate) mod state;
use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new()?;
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
