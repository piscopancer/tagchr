use std::sync::mpsc::Sender;
use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Color, Stylize },
  text::{ Line, Span, Text },
  widgets::{ Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Widget, WidgetRef },
  Frame,
};
use crate::{ app::{ app::Command, state::State }, ui::{ InputHandler, UiState } };
use super::{ help::HelpModal, save_result::SaveTagsResultModal, save_tags::ConfirmSaveTagsModal };

pub mod enums {
  use crate::ui::modals::{
    help::HelpModal,
    save_result::SaveTagsResultModal,
    save_tags::ConfirmSaveTagsModal,
  };

  pub enum Modal {
    ConfirmSaveTags(ConfirmSaveTagsModal),
    SaveTagsResult(SaveTagsResultModal),
    Help(HelpModal),
  }
}

pub struct ModalOptions {
  current: usize,
  list: Vec<ModalOption>,
}

impl ModalOptions {
  pub fn select(&mut self, i: usize) {
    self.current = i;
  }
}

impl From<&ModalOptions> for Table<'_> {
  fn from(options: &ModalOptions) -> Self {
    Table::new(
      Vec::from([
        Row::new(
          options.list
            .iter()
            .enumerate()
            .map(|(i, o)| (
              if options.current() == i {
                Cell::new(Text::from(o.desc.clone()).centered()).reversed()
              } else {
                Cell::new(Text::from(o.desc.clone()).centered())
              }
            ))
        ),
      ]),
      (0..options.list.len()).map(|_| Constraint::Fill(1))
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
  pub fn next(&self) -> usize {
    if self.current == self.list.len() - 1 { 0 } else { self.current + 1 }
  }
  pub fn prev(&self) -> usize {
    if self.current == 0 { self.list.len() - 1 } else { self.current - 1 }
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
}

pub struct ModalOption {
  pub desc: String,
  pub cmd: Option<Command>,
}

impl ModalOption {
  pub fn new(desc: impl Into<String>, cmd: Command) -> Self {
    Self {
      desc: desc.into(),
      cmd: Some(cmd),
    }
  }
}

pub trait Modal: WidgetRef + InputHandler {
  fn options(&self) -> Option<&ModalOptions>;
  fn options_mut(&mut self) -> Option<&mut ModalOptions>;
}

pub struct Modals(Vec<Box<dyn Modal>>);

impl Modals {
  pub fn new() -> Self {
    // TODO: remove debug
    // let debug = false;
    // if debug {
    //   let mut new = Self(Vec::new());
    //   new.open(enums::Modal::ConfirmSaveTags {
    //     index: 0,
    //     song_title: "SUCK ME".into(),
    //   });
    //   new
    // } else {
    Self(Vec::new())
    // }
  }
  pub fn open(&mut self, modal: enums::Modal) {
    self.0.push(match modal {
      enums::Modal::Help(modal) => Box::new(modal),
      enums::Modal::ConfirmSaveTags(modal) => Box::new(modal),
      enums::Modal::SaveTagsResult(modal) => Box::new(modal),
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
  pub fn last(&self) -> Option<&Box<dyn Modal>> {
    self.0.last()
  }
  pub fn last_mut(&mut self) -> Option<&mut Box<dyn Modal>> {
    self.0.last_mut()
  }
  pub fn close_all(&mut self) {
    self.0 = Vec::new();
  }
}
