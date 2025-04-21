use little_exif::metadata::Metadata as ExifMetadata;
use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};
use std::path::PathBuf;

use crate::exif_data::constant_metadata::ConstantMetadata;

pub struct ConstantExifData {
    image_path: PathBuf,
}

impl ConstantExifData {
    pub fn new(image_path: PathBuf) -> Self {
        Self { image_path }
    }
}

impl Widget for &ConstantExifData {
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

        let exif = ExifMetadata::new_from_path(&self.image_path).expect("Failed to read EXIF data");
        let metadata = ConstantMetadata::from_exif(&exif).unwrap();

        Paragraph::new(metadata.to_string()).render(image_area, buf);
    }
}
