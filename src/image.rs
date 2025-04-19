use ratatui::{layout::Rect, widgets::StatefulWidget};
use ratatui_image::{protocol::StatefulProtocol, StatefulImage};

#[derive(Default)]
pub struct Image {}

impl StatefulWidget for &Image {
    type State = StatefulProtocol;

    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let image_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );
        let image = StatefulImage::default();
        image.render(image_area, buf, state);
    }
}
