use ratatui::{ buffer::Buffer, layout::Rect, text::Line, widgets::{ Widget, WidgetRef } };
use super::modal::Modal;

pub struct SaveResultModal;
impl Modal for SaveResultModal {}
impl WidgetRef for SaveResultModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    // Line::from("Saved").render(area, buf);
  }
}
