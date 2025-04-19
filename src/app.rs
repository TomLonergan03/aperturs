use std::path::PathBuf;

use crate::{image::Image, state::AppState};
use image::ImageReader;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};

pub struct App {
    exit: bool,
    image: StatefulProtocol,
    current_image: PathBuf,
    state: AppState,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Should use Picker::from_query_stdio() to get the font size and protocol,
        // but we can't put that here because that would break doctests!
        let picker = Picker::from_query_stdio()?;

        // Load an image with the image crate.
        let dyn_img = ImageReader::open("test_image.jpg")?.decode()?;

        // Create the Protocol which will be used by the widget.
        let image = picker.new_resize_protocol(dyn_img);
        Ok(Self {
            exit: false,
            image,
            current_image: PathBuf::from("test_image.jpg"),
            state: AppState::ActiveImage,
        })
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget(self, frame.area(), &mut ());
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                self.next_image()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_image(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let picker = Picker::from_query_stdio()?;
        self.current_image = if self.current_image == PathBuf::from("test_image.jpg") {
            PathBuf::from("test_image_2.jpg")
        } else {
            PathBuf::from("test_image.jpg")
        };
        let image = ImageReader::open(&self.current_image)?.decode()?;
        self.image = picker.new_resize_protocol(image);
        Ok(())
    }
}

impl StatefulWidget for &mut App {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, _: &mut Self::State) {
        let title = Line::from(" Aperturs ".bold());
        let instructions = Line::from(vec![
            " Next image ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        block.render(area, buf);

        let app_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(app_area);

        let metadata_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(layout[0]);

        let image_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(layout[1]);

        match self.state {
            AppState::FolderSelection => render_folder_selection(area, buf),
            AppState::Initialising => todo!(),
            AppState::NoActiveImage => todo!(),
            AppState::ActiveImage => {
                Paragraph::new(Text::from("Images go here".bold()))
                    .block(Block::new().borders(Borders::ALL).title("Images"))
                    .render(metadata_layout[0], buf);

                Paragraph::new(Text::from("EXIF data here".bold()))
                    .block(Block::new().borders(Borders::ALL).title("EXIF data"))
                    .render(metadata_layout[1], buf);

                Image::default().render(image_layout[0], buf, &mut self.image);

                Paragraph::new(Text::from("Some dynamic stuff maybe".bold()))
                    .block(Block::new().borders(Borders::ALL).title("Placeholder"))
                    .render(image_layout[1], buf);
            }
        };
    }
}

fn render_folder_selection(area: Rect, buf: &mut Buffer) {
    Paragraph::new(Text::from("Select a folder".bold()))
        .block(Block::new().borders(Borders::ALL).title("Placeholder"))
        .render(area, buf);
}
