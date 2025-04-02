use crossterm::event::{ KeyCode, KeyEvent };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Color, Stylize },
  text::{ Line, Span, Text },
  widgets::{ Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Widget, WidgetRef },
};
use crate::{ app::app::State, ui::UiCommand };
use super::modal::{ Modal, ModalOption, ModalOptions };

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
        ModalOption::new("Save", |s| {
          s.searched_mp3_files[index].tags.save();
          None
        }),
        ModalOption::new("Cancel", |s| { None }),
      ]),
    }
  }
}

impl Modal for ConfirmSaveTagsModal {
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Option<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, ..) => {
        return Some(UiCommand::CloseLastModal);
      }
      (KeyCode::Left, ..) => {
        self.options.prev();
      }
      (KeyCode::Right, ..) => {
        self.options.next();
      }
      (KeyCode::Enter, ..) => {
        (self.options.list()[self.options.current()].action)(state);
        return Some(UiCommand::CloseLastModal);
      }
      (KeyCode::Char('s' | 'с'), ..) => {
        (self.options.list()[0].action)(state);
        return Some(UiCommand::CloseLastModal);
      }
      (KeyCode::Right, ..) => {
        return Some(UiCommand::CloseLastModal);
      }
      _ => {}
    }
    None
  }
}

impl WidgetRef for ConfirmSaveTagsModal {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) where Self: Sized {
    let [modal_area] = Layout::vertical([Constraint::Max(10)])
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
      .title_top(Line::from(" Confirmation ").centered())
      .render(modal_area, buf);
    Paragraph::new(
      Line::from({
        Vec::from([
          Span::from("ID3 tags will be saved for "),
          Span::from(self.song_title.clone()).yellow(),
        ])
      })
    )
      .centered()
      .render(
        Rect {
          x: modal_area.x + 1,
          y: modal_area.y + 2,
          width: modal_area.width - 2,
          height: modal_area.height - 6,
        },
        buf
      );
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
    Table::new(
      Vec::from([
        Row::new(
          self.options
            .list()
            .iter()
            .enumerate()
            .map(|(i, o)| (
              if self.options.current() == i {
                Cell::new(Text::from(o.desc.clone()).centered()).reversed()
              } else {
                Cell::new(Text::from(o.desc.clone()).centered())
              }
            ))
        ),
      ]),
      (0..self.options.list().len()).map(|_| Constraint::Fill(1))
    )
      .column_spacing(1)
      .render(options_area.inner(Margin::new(2, 0)).offset(Offset { x: 0, y: -1 }), buf);
  }
}
