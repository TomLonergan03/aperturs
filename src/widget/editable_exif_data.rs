use crate::exif_data::editable_metadata::EditableMetadata;
use little_exif::metadata::Metadata as ExifMetadata;
use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};
use std::path::PathBuf;

pub struct EditableExifData {
    image_path: PathBuf,
}

impl EditableExifData {
    pub fn new(image_path: PathBuf) -> Self {
        Self { image_path }
    }
}

impl Widget for &EditableExifData {
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
        let metadata = EditableMetadata::from_exif(&exif, &self.image_path).unwrap();

        Paragraph::new(metadata.to_string()).render(image_area, buf);
    }
}
