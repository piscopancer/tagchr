use ratatui::{ style::Stylize, text::Span };

pub struct Shortcut {
  key: String,
  title: String,
}

type ShortcutOutput = Vec<Span<'static>>;

impl Shortcut {
  pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
    Self {
      key: key.into(),
      title: title.into(),
    }
  }
  pub fn to_spans(&self) -> ShortcutOutput {
    Vec::from([
      Span::from(" ").on_light_magenta(),
      Span::from(self.key.clone()).on_light_magenta().bold().black(),
      Span::from(" ").on_light_magenta(),
      Span::from("|").dark_gray(),
      Span::from(self.title.clone()).underlined(),
    ])
  }
  pub fn spans_length(&self) -> usize {
    1 + self.key.chars().count() + 1 + 1 + self.title.chars().count()
  }
}
