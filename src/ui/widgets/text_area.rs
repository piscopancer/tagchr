use ratatui::{ style::{ Style, Stylize }, widgets::{ Block, BorderType } };
use tui_textarea::{ CursorMove, Input, Key, TextArea };
use crate::ui::{ StringTrait, StyleFlags };
use super::{
  block::BlockTrait,
  widget::{ FocusableWidget, ToggleableWidget, WidgetWithEditableContent },
};
use clipboard_win::{ get_clipboard_string, set_clipboard_string };

pub trait TextAreaTrait {
  fn clear(&mut self);
  fn set_text(&mut self, text: String);
  fn toggle_cursor(&mut self, t: bool);
  fn input_for_humans(&mut self, input: impl Into<Input>, multiline: bool) -> bool;
}

impl TextAreaTrait for TextArea<'_> {
  fn clear(&mut self) {
    self.select_all();
    self.delete_char();
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
  fn input_for_humans(&mut self, input: impl Into<Input>, multiline: bool) -> bool {
    match input.into() {
      Input { key: Key::Char(c), ctrl: false, alt: false, .. } => {
        self.insert_char(c);
        true
      }
      Input { key: Key::Char('c'), ctrl: true, alt: false, .. } | Input { key: Key::Copy, .. } => {
        self.copy();
        set_clipboard_string(&self.yank_text());
        false
      }
      Input { key: Key::Char('v'), ctrl: true, alt: false, .. } => {
        let text = get_clipboard_string();
        if text.is_err() {
          return false;
        }
        let mut text = text.unwrap();
        let text = if multiline { text } else { text.to_single_line() };
        self.insert_str(text);
        true
      }
      Input { key: Key::Tab, ctrl: false, alt: false, .. } => self.insert_tab(),
      Input { key: Key::Backspace, ctrl: false, alt: true, .. } => self.delete_word(),
      Input { key: Key::Backspace, .. } => self.delete_char(),
      Input { key: Key::Delete, .. } => self.delete_next_char(),
      Input { key: Key::Enter, .. } if multiline => {
        self.insert_newline();
        true
      }
      Input { key: Key::Right, ctrl: false, alt: false, .. } => {
        self.move_cursor(CursorMove::Forward);
        false
      }
      Input { key: Key::Left, ctrl: false, alt: false, .. } => {
        self.move_cursor(CursorMove::Back);
        false
      }
      Input { key: Key::MouseScrollDown, .. } => {
        self.scroll((1, 0));
        false
      }
      Input { key: Key::MouseScrollUp, .. } => {
        self.scroll((-1, 0));
        false
      }
      _ => false,
    }
  }
}
