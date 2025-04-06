use crossterm::event::{ KeyCode, KeyEvent };
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
use crate::{ app::app::State, ui::UiCommand };
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
        ModalOption::new(
          "Save",
          Box::new(move |state| {
            let res = state.searched_mp3_files[index].tags.save();
            Vec::from([
              UiCommand::CloseLastModal,
              UiCommand::OpenModal(modal::enums::Modal::SaveResult(res)),
            ])
          })
        ),
        ModalOption::new(
          "Cancel",
          Box::new(|state| Vec::from([UiCommand::CloseLastModal]))
        ),
      ]),
    }
  }
}

impl Modal for ConfirmSaveTagsModal {
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Vec<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, ..) => {
        return Vec::from([UiCommand::CloseLastModal]);
      }
      (KeyCode::Left, ..) => {
        self.options.prev();
      }
      (KeyCode::Right, ..) => {
        self.options.next();
      }
      (KeyCode::Enter, ..) => {
        return self.options.exec_current(state);
      }
      (KeyCode::Char('s' | 'с'), ..) => {
        return self.options.list_mut()[0].exec(state);
      }
      _ => {}
    }
    Vec::new()
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
