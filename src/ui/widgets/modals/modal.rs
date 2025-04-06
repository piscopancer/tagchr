use crossterm::event::{ KeyCode, KeyEvent };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Color, Stylize },
  text::{ Line, Span, Text },
  widgets::{ Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Widget, WidgetRef },
  Frame,
};
use crate::{ app::app::State, ui::UiCommand };
use super::{ help::HelpModal, save_result::SaveResultModal, save_tags::ConfirmSaveTagsModal };

pub mod enums {
  pub enum Modal {
    Save {
      index: usize,
      song_title: String,
    },
    SaveResult(Result<(), String>),
    Help,
  }
}

pub struct ModalOptions {
  current: usize,
  list: Vec<ModalOption>,
}

impl From<&ModalOptions> for Table<'_> {
  fn from(value: &ModalOptions) -> Self {
    Table::new(
      Vec::from([
        Row::new(
          value.list
            .iter()
            .enumerate()
            .map(|(i, o)| (
              if value.current() == i {
                Cell::new(Text::from(o.desc.clone()).centered()).reversed()
              } else {
                Cell::new(Text::from(o.desc.clone()).centered())
              }
            ))
        ),
      ]),
      (0..value.list.len()).map(|_| Constraint::Fill(1))
    ).column_spacing(1)
  }
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
  pub fn list_mut(&mut self) -> &mut Vec<ModalOption> {
    &mut self.list
  }
  pub fn current(&self) -> usize {
    self.current
  }
  pub fn exec_current(&mut self, state: &mut State) -> Vec<UiCommand> {
    let i = self.current();
    let cmd = self.list_mut()[i].exec(state);
    cmd
  }
}

type Action = Box<dyn FnMut(&mut State) -> Vec<UiCommand>>;

pub struct ModalOption {
  pub desc: String,
  action: Action,
}

impl ModalOption {
  pub fn new(desc: impl Into<String>, action: Action) -> Self {
    Self {
      desc: desc.into(),
      action,
    }
  }
  pub fn exec(&mut self, state: &mut State) -> Vec<UiCommand> {
    (self.action)(state)
  }
}

pub trait Modal: WidgetRef {
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Vec<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc | KeyCode::Enter | KeyCode::Backspace | KeyCode::Char(' '), ..) => {
        return Vec::from([UiCommand::CloseLastModal]);
      }
      _ => {}
    }
    Vec::new()
  }
}

pub struct Modals(Vec<Box<dyn Modal>>);

impl Modals {
  pub fn new() -> Self {
    // TODO: remove debug
    let debug = false;
    if debug {
      let mut new = Self(Vec::new());
      new.open(enums::Modal::Save {
        index: 0,
        song_title: "SUCK ME".into(),
      });
      new
    } else {
      Self(Vec::new())
    }
  }
  pub fn open(&mut self, modal: enums::Modal) {
    self.0.push(match modal {
      enums::Modal::Help => Box::new(HelpModal),
      enums::Modal::Save { index, song_title } =>
        Box::new(ConfirmSaveTagsModal::new(index, song_title)),
      enums::Modal::SaveResult(_) => Box::new(SaveResultModal::new()),
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
