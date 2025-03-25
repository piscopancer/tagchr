use crossterm::event::KeyEvent;
use ratatui::{ style::Stylize, text::Line, widgets::Paragraph, Frame };

use crate::{ app::app::App, ui::ui::Screen };

pub struct LyricsScreen {}

impl LyricsScreen {
  pub fn new() -> Self {
    Self {}
  }
}

impl Screen for LyricsScreen {
  fn draw(&mut self, frame: &mut Frame, app: &App) {
    let p = Paragraph::new(vec![Line::from("".red())]);
    frame.render_widget(&p, frame.area());
  }
  fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
    todo!()
  }
}
