use crossterm::event::KeyEvent;
use id3::frame::Lyrics;
use ratatui::{ style::Stylize, text::Line, widgets::Paragraph, Frame };
use crate::{
  app::app::{ App, EditableTag, SongTags, State },
  ui::ui::{ ui_enums, Screen, UiCommand },
};

pub struct LyricsScreen {
  lyrics: EditableTag<Lyrics>,
}

impl LyricsScreen {
  pub fn new(lyrics: EditableTag<Lyrics>) -> Self {
    Self {
      lyrics,
    }
  }
}

impl LyricsScreen {
  pub fn edited_lyrics(&self, tags: &mut SongTags) -> EditableTag<Lyrics> {
    tags.lyrics.clone()
  }
}

impl Screen for LyricsScreen {
  fn draw(&mut self, frame: &mut Frame, app: &State) {
    let p = Paragraph::new(vec![Line::from("LOL".red())]);
    frame.render_widget(&p, frame.area());
  }
  fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut State) -> Option<UiCommand> {
    None
  }
}
