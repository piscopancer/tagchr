use std::sync::mpsc::Sender;

use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Style, Stylize },
  text::Line,
  widgets::{ Block, BorderType, Borders, Clear, Paragraph, Table, Widget, WidgetRef },
};
use crate::{ app::{ app::Command, state::State }, ui::{ InputHandler, UiState } };

use super::modal::{ Modal, ModalOption, ModalOptions };

pub struct SaveTagsResultModal {
  res: String,
  pub options: ModalOptions,
}

impl SaveTagsResultModal {
  pub fn new(res: Result<(), String>) -> Self {
    Self {
      res: match res.clone() {
        Ok(_) => "Saved".into(),
        Err(err) => "Something went wrong".into(),
      },
      options: ModalOptions::new([
        ModalOption::new(
          match res {
            Ok(_) => "Cool".to_string(),
            Err(_) => ":(".to_string(),
          },
          Command::CloseLastModal
        ),
      ]),
    }
  }
}

impl Modal for SaveTagsResultModal {
  fn options(&self) -> Option<&ModalOptions> {
    Some(&self.options)
  }
  fn options_mut(&mut self) -> Option<&mut ModalOptions> {
    Some(&mut self.options)
  }
}

impl InputHandler for SaveTagsResultModal {
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
          KeyCode::Esc | KeyCode::Enter | KeyCode::Backspace => {
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

impl WidgetRef for SaveTagsResultModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    let [area] = Layout::vertical([Constraint::Max(7)])
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
    Block::bordered().border_type(BorderType::Rounded).render(area, buf);
    Paragraph::new(Line::from(self.res.clone()).centered()).render(
      content_area.inner(Margin::new(1, 1)),
      buf
    );
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
    Table::from(&self.options).render(options_area, buf);
  }
}
