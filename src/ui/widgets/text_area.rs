use ratatui::{ style::{ Style, Stylize }, widgets::{ Block, BorderType } };
use tui_textarea::{ Input, TextArea };
use crate::ui::WidgetState;
use super::{
  block::BlockTrait,
  widget::{ FocusableWidget, ToggleableWidget, WidgetWithEditableContent },
};

pub trait TextAreaTrait {
  fn clear(&mut self);
  fn set_text(&mut self, text: String);
  fn toggle_cursor(&mut self, t: bool);
  fn text_as_single_line(&self) -> String;
}

impl TextAreaTrait for TextArea<'_> {
  fn clear(&mut self) {
    *self = TextArea::default();
  }
  fn set_text(&mut self, text: String) {
    self.clear();
    self.insert_str(text);
  }
  fn toggle_cursor(&mut self, t: bool) {
    self.set_cursor_style(
      if t {
        self.cursor_style().reversed()
      } else {
        self.cursor_style().reset()
      }
    );
  }
  fn text_as_single_line(&self) -> String {
    self.lines().clone().join(" ")
  }
}
