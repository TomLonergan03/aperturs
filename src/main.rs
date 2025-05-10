pub(crate) mod exif_data;
mod state;
mod widget;
use widget::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new()?;
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
