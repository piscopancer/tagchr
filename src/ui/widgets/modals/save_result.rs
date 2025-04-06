use crossterm::event::{ KeyCode, KeyEvent };
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Offset, Rect },
  style::{ Style, Stylize },
  text::Line,
  widgets::{ Block, BorderType, Borders, Clear, Paragraph, Table, Widget, WidgetRef },
};
use crate::{ app::app::State, ui::UiCommand };
use super::modal::{ Modal, ModalOption, ModalOptions };

pub struct SaveResultModal {
  options: ModalOptions,
}

impl SaveResultModal {
  pub fn new() -> Self {
    Self {
      options: ModalOptions::new([
        ModalOption::new(
          "Cool",
          Box::new(|state| { Vec::from([UiCommand::CloseLastModal]) })
        ),
      ]),
    }
  }
}

impl Modal for SaveResultModal {}

impl WidgetRef for SaveResultModal {
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
    Paragraph::new(Line::from("Saved").centered()).render(
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
