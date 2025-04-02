use ratatui::{ buffer::Buffer, layout::Rect, widgets::WidgetRef };
use super::modal::Modal;

pub struct HelpModal;
impl Modal for HelpModal {}
impl WidgetRef for HelpModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    // Line::raw("Hello").render(area, buf);
  }
}
