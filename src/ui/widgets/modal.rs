use crossterm::event::KeyEvent;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Rect },
  text::{ Line, Text },
  widgets::{ Block, BorderType, Borders, Clear, Paragraph, Widget },
};
use crate::app::{ app::State, tag::SongTags };

struct ModalOption {
  desc: String,
  action: Box<dyn Fn()>,
}

impl ModalOption {
  fn new(desc: impl Into<String>, action: impl Fn() + 'static) -> Self {
    Self {
      desc: desc.into(),
      action: Box::new(action),
    }
  }
}

pub struct Modal {
  title: Option<String>,
  content: Option<Text<'static>>,
  options: Vec<ModalOption>,
}

mod enums {
  pub enum Modal {
    SaveSongTags(usize),
    SaveSongTagsResult(Result<String, String>),
    Help,
  }
}

impl Modal {
  pub fn new() -> Self {
    Self {
      title: None,
      content: None,
      options: Vec::new(),
    }
  }
  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.title = Some(title.into());
    self
  }
  pub fn content<C>(mut self, content: C) -> Self where C: Into<Text<'static>> {
    self.content = Some(content.into());
    self
  }
  pub fn option(mut self, option: ModalOption) -> Self {
    self.options.push(option);
    self
  }
  // (TODO: put this shit into option if possible) pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {}
}

impl Widget for &Modal {
  fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
    let [modal_area] = Layout::vertical([Constraint::Max(20)])
      .flex(Flex::Center)
      .areas(
        Layout::horizontal([Constraint::Max(40)])
          .flex(Flex::Center)
          .areas::<1>(area)[0]
      );
    let [content_area, options_area] = Layout::vertical([
      Constraint::Fill(1),
      Constraint::Length(1),
    ])
      .spacing(1)
      .areas(modal_area);

    Clear.render(modal_area, buf);
    Block::bordered()
      .border_type(BorderType::Rounded)
      .title_top(Line::from(self.title.clone().unwrap_or_default()).centered())
      .render(modal_area, buf);
    Block::bordered()
      .borders(Borders::TOP)
      .render(
        Rect {
          x: modal_area.x + 1,
          y: modal_area.y + modal_area.height - 3,
          width: modal_area.width - 2,
          height: 1,
        },
        buf
      );
  }
}

pub struct Modals(Vec<Modal>);

impl Modals {
  pub fn new() -> Self {
    // Self(Vec::new());
    let mut new = Self(Vec::new());
    new.open(enums::Modal::SaveSongTags(0));
    new
  }
  pub fn open(&mut self, modal: enums::Modal) {
    self.0.push(match modal {
      enums::Modal::SaveSongTags(i) =>
        Modal::new()
          .title(" Save song tags ")
          .option(
            ModalOption::new(
              "Save",
              || {
                // TODO: save using state passed in here
              }
            )
          )
          .option(ModalOption::new("Cancel", || {})),

      enums::Modal::SaveSongTagsResult(_) => Modal::new(),
      enums::Modal::Help => Modal::new(),
    });
  }
  pub fn iter(&self) -> impl Iterator<Item = &Modal> {
    self.0.iter()
  }
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Modal> {
    self.0.iter_mut()
  }
  fn close_last(&mut self) {
    // self.last()
  }
  // pub fn close_if(&mut self, cond: bool) -> bool {}
  pub fn last(&mut self) -> Option<&mut Modal> {
    self.0.last_mut()
  }
  pub fn close_all(&mut self) {
    self.0 = Vec::new();
  }
}
