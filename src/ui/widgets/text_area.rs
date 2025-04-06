use ratatui::{ style::{ Style, Stylize }, widgets::{ Block, BorderType } };
use tui_textarea::{ CursorMove, Input, Key, TextArea };
use crate::ui::{ StringTrait, WidgetState };
use super::{
  block::BlockTrait,
  widget::{ FocusableWidget, ToggleableWidget, WidgetWithEditableContent },
};
use clipboard_win::{ get_clipboard_string, set_clipboard_string };

pub trait TextAreaTrait {
  fn custom() -> Self;
  fn clear(&mut self);
  fn set_text(&mut self, text: String);
  fn toggle_cursor(&mut self, t: bool);
  fn input_for_humans(&mut self, input: impl Into<Input>, multiline: bool) -> bool;
}

impl TextAreaTrait for TextArea<'_> {
  fn custom() -> Self {
    let mut new = Self::default();
    new.set_cursor_line_style(Style::new());
    new
  }
  fn clear(&mut self) {
    *self = TextArea::custom();
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

// trait Validation<O, E> {
//   fn validate(validator: impl Fn() -> bool) -> Result<O, E>;
// }

// struct InputValidation {

// }

// impl InputValidation {

// }

// impl Validation for InputValidation {
//   type Ok = ;
// }

/* 
input_state
  toggleation
  validation

button_state
  toggleation

text_area_state
  scroll
*/

//

struct Validation<F> {
  validator: F,
}

impl<F> Validation<F> {
  pub fn new(validator: F) -> Self {
    Self {
      validator,
    }
  }
  pub fn verify(&self) -> bool where F: Fn() -> bool {
    (self.validator)()
  }
}

//

type InputValidation = Validation<String>;

struct InputState {
  flags: WidgetState,
  validation: Option<InputValidation>,
}

impl InputState {
  pub fn new() -> Self {
    Self {
      flags: WidgetState::empty(),
      validation: None,
    }
  }
  pub fn with_validation(mut self, v: InputValidation) -> Self {
    self.validation = Some(v);
    self
  }
}
