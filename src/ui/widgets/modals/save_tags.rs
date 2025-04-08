use std::sync::mpsc::Sender;

use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Color, Style, Stylize },
  text::{ Line, Span, Text },
  widgets::{
    Block,
    BorderType,
    Borders,
    Cell,
    Clear,
    Paragraph,
    Row,
    Table,
    Widget,
    WidgetRef,
    Wrap,
  },
};
use crate::{ app::{ app::Command, state::State }, ui::{ InputHandler, UiState } };
use super::modal::{ self, Modal, ModalOption, ModalOptions };

pub struct ConfirmSaveTagsModal {
  index: usize,
  song_title: String,
  options: ModalOptions,
}

impl ConfirmSaveTagsModal {
  pub fn new(index: usize, song_title: impl Into<String>) -> Self {
    Self {
      index,
      song_title: song_title.into(),
      options: ModalOptions::new([
        ModalOption::new("Save", Command::SaveTags(index)),
        ModalOption::new("Cancel", Command::CloseLastModal),
      ]),
    }
  }
}

impl InputHandler for ConfirmSaveTagsModal {
  fn handle_input(
    &self,
    state: &State,
    ui_state: &UiState,
    event: Event,
    sender: Sender<Command>
  ) -> bool {
    match event {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        match key_event.code {
          KeyCode::Esc => {
            sender.send(Command::CloseLastModal);
            true
          }
          KeyCode::Left => {
            sender.send(Command::SetModalOption(self.options.prev()));
            true
          }
          KeyCode::Right => {
            sender.send(Command::SetModalOption(self.options.next()));
            true
          }
          KeyCode::Enter => {
            sender.send(Command::ExecuteModalOption(self.options.current()));
            sender.send(Command::CloseLastModal);
            true
          }
          _ => false,
        }
      }
      _ => false,
    }
  }
}

impl Modal for ConfirmSaveTagsModal {
  fn options(&self) -> Option<&ModalOptions> {
    Some(&self.options)
  }
  fn options_mut(&mut self) -> Option<&mut ModalOptions> {
    Some(&mut self.options)
  }
}

impl WidgetRef for ConfirmSaveTagsModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) where Self: Sized {
    let [area] = Layout::vertical([Constraint::Max(10)])
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
      .areas(area.inner(Margin::new(1, 1)));

    Clear.render(area, buf);
    Block::bordered()
      .border_type(BorderType::Rounded)
      .title_top(Line::from(" Confirmation ").centered())
      .render(area, buf);
    Paragraph::new(
      Vec::from([
        Line::default(),
        Line::from({
          Vec::from([
            Span::from("ID3 tags will be saved for "),
            Span::from(self.song_title.clone()).yellow(),
          ])
        }),
      ])
    )
      .centered()
      .wrap(Wrap::default())
      .render(area.inner(Margin::new(1, 1)), buf);
    Block::bordered()
      .borders(Borders::TOP)
      .border_style(Style::new())
      .render(
        Rect {
          x: content_area.x,
          y: content_area.y + content_area.height,
          width: content_area.width,
          height: 1,
        },
        buf
      );
    Table::from(&self.options).render(options_area.inner(Margin::new(1, 0)), buf);
  }
}
