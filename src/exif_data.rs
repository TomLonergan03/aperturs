use std::{path::PathBuf, vec};

use little_exif::metadata::Metadata;
use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

pub struct ExifData {
    image_path: PathBuf,
}

impl ExifData {
    pub fn new(image_path: PathBuf) -> Self {
        Self { image_path }
    }
}

impl Widget for &ExifData {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        Block::default()
            .title("Exif Data")
            .borders(ratatui::widgets::Borders::ALL)
            .render(area, buf);

        let image_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );

        let metadata = Metadata::new_from_path(&self.image_path).expect("Failed to read EXIF data");
        let mut exif_data = String::new();
        for tag in &metadata {
            exif_data.push_str(&(format!("{:?}", tag) + "\n"));
        }

        Paragraph::new(exif_data).render(image_area, buf);
    }
}
