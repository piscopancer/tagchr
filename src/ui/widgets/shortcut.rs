use ratatui::{ style::{ Color, Stylize }, text::Span };

pub struct Shortcut {
  key: String,
  title: String,
  color: Color,
}

type ShortcutOutput = Vec<Span<'static>>;

impl Shortcut {
  pub fn new(key: impl Into<String>, title: impl Into<String>, color: Color) -> Self {
    Self {
      key: key.into(),
      title: title.into(),
      color,
    }
  }
  pub fn to_spans(&self) -> ShortcutOutput {
    Vec::from([
      Span::from(" ").bg(self.color),
      Span::from(self.key.clone()).bold().black().bg(self.color),
      Span::from(" ").bg(self.color),
      Span::from("|").dark_gray(),
      Span::from(self.title.clone()).underlined(),
    ])
  }
  pub fn spans_length(&self) -> usize {
    1 + self.key.chars().count() + 1 + 1 + self.title.chars().count()
  }
}
