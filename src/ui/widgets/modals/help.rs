use std::sync::mpsc::Sender;

use crossterm::event::{ Event, KeyCode, KeyEventKind };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Rect },
  style::Stylize,
  text::{ Line, Span },
  widgets::{ block::Title, Block, BorderType, Clear, Paragraph, Widget, WidgetRef, Wrap },
};
use crate::{ app::{ app::Command, state::State }, ui::{ InputHandler, UiState } };

use super::modal::Modal;

pub struct HelpModal;

impl Modal for HelpModal {
  fn options(&self) -> Option<&super::modal::ModalOptions> {
    None
  }
  fn options_mut(&mut self) -> Option<&mut super::modal::ModalOptions> {
    None
  }
}

impl InputHandler for HelpModal {
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

impl WidgetRef for HelpModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    let [area] = Layout::vertical([Constraint::Max(10)])
      .flex(Flex::Center)
      .areas(
        Layout::horizontal([Constraint::Max(40)])
          .flex(Flex::Center)
          .areas::<1>(area)[0]
      );

    Clear.render(area, buf);
    Block::bordered()
      .border_type(BorderType::Rounded)
      .title(Title::from(Line::from(" Help ").centered()))
      .render(area, buf);
    Paragraph::new(
      Line::from(
        Vec::from([
          Span::from("Choose MP3 file from the list to start "),
          Span::from("editing").yellow(),
        ])
      )
    )
      .centered()
      .wrap(Wrap::default())
      .render(area.inner(Margin::new(2, 2)), buf);
  }
}
