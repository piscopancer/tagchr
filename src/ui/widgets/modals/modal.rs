use crossterm::event::{ KeyCode, KeyEvent };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Color, Stylize },
  text::{ Line, Span },
  widgets::{ Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Widget, WidgetRef },
  Frame,
};
use crate::{ app::app::State, ui::UiCommand };
use super::{ help::HelpModal, save_tags::ConfirmSaveTagsModal };

pub mod enums {
  pub enum Modal {
    Save {
      index: usize,
      song_title: String,
    },
    SaveResult(Result<String, String>),
    Help,
  }
}

pub struct ModalOptions {
  current: usize,
  list: Vec<ModalOption>,
}

impl ModalOptions {
  pub fn new(options: impl Into<Vec<ModalOption>>) -> Self {
    Self {
      current: 0,
      list: options.into(),
    }
  }
  pub fn next(&mut self) {
    self.current = if self.current == self.list.len() - 1 { 0 } else { self.current + 1 };
  }
  pub fn prev(&mut self) {
    self.current = if self.current == 0 { self.list.len() - 1 } else { self.current - 1 };
  }
  pub fn list(&self) -> &Vec<ModalOption> {
    &self.list
  }
  pub fn current(&self) -> usize {
    self.current
  }
}

type Action = fn(&mut State) -> Option<UiCommand>;

pub struct ModalOption {
  pub desc: String,
  pub action: Action,
}

impl ModalOption {
  pub fn new(desc: impl Into<String>, action: Action) -> Self {
    Self {
      desc: desc.into(),
      action,
    }
  }
}

pub trait Modal: WidgetRef {
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Option<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, ..) => {
        return Some(UiCommand::CloseLastModal);
      }
      _ => {}
    }
    None
  }
}

pub struct Modals(Vec<Box<dyn Modal>>);

impl Modals {
  pub fn new() -> Self {
    // Self(Vec::new());
    let mut new = Self(Vec::new());
    new.open(enums::Modal::Save {
      index: 0,
      song_title: "SUCK ME".into(),
    });
    new
  }
  pub fn open(&mut self, modal: enums::Modal) {
    self.0.push(match modal {
      enums::Modal::Help => Box::new(HelpModal),
      enums::Modal::Save { index, song_title } =>
        Box::new(ConfirmSaveTagsModal::new(index, song_title)),
      enums::Modal::SaveResult(_) => todo!(),
    });
  }
  pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Modal>> {
    self.0.iter()
  }
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Modal>> {
    self.0.iter_mut()
  }
  pub fn close_last(&mut self) {
    self.0.pop();
  }
  pub fn last(&mut self) -> Option<&mut Box<dyn Modal>> {
    self.0.last_mut()
  }
  pub fn close_all(&mut self) {
    self.0 = Vec::new();
  }
}
