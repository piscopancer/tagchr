use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use ratatui::{
  style::{ Color, Style, Styled, Stylize },
  widgets::{ Block, BorderType, List, Table },
  Frame,
};
use tui_textarea::TextArea;
use crate::app::app::App;
use super::{ home::screen::HomeScreen, lyrics::screen::LyricsScreen };

pub mod ui_enums {
  pub enum Screen {
    Home,
    Lyrics,
  }
}

pub enum UiCommand {
  ChangeScreen(ui_enums::Screen),
}

pub struct Ui {
  screen: ui_enums::Screen,
  home_screen: HomeScreen,
  lyrics_screen: LyricsScreen,
}

impl Ui {
  pub fn new() -> Self {
    Self {
      screen: ui_enums::Screen::Home,
      home_screen: HomeScreen::new(),
      lyrics_screen: LyricsScreen::new(),
    }
  }
  pub fn draw(&mut self, frame: &mut Frame, app: &App) {
    match self.screen {
      ui_enums::Screen::Home => {
        self.home_screen.draw(frame, app);
      }
      ui_enums::Screen::Lyrics => {
        self.lyrics_screen.draw(frame, app);
      }
    }
  }
  pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, _) => {
        app.running = false;
      }
      (code, modifiers) => {
        match self.screen {
          ui_enums::Screen::Home => {
            if let Some(cmd) = self.home_screen.handle_key_event(key_event, app) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  self.screen = screen;
                }
              }
            }
          }
          ui_enums::Screen::Lyrics => {
            if let Some(cmd) = self.lyrics_screen.handle_key_event(key_event, app) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  self.screen = screen;
                }
              }
            }
          }
        };
      }
    }
  }
}

pub trait Screen {
  fn draw(&mut self, frame: &mut Frame, app: &App);
  fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) -> Option<UiCommand>;
}

pub fn basic_text_area(title: String) -> TextArea<'static> {
  let mut input = TextArea::default();
  input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top(title));
  input
}

pub trait TableTrait {
  fn created_highlighted(&mut self, prev_block: Block<'static>, v: bool) -> Self;
}

impl TableTrait for Table<'_> {
  fn created_highlighted(&mut self, prev_block: Block<'static>, v: bool) -> Self {
    self.clone().block(
      Block::from(prev_block)
        .title_style(if v { Style::new().yellow() } else { Style::new().reset() })
        .border_style(if v { Style::new().yellow() } else { Style::new().reset() })
    )
  }
}

pub trait BlockTrait {
  fn highlighted(&self, v: bool) -> Self;
}

impl BlockTrait for Block<'_> {
  fn highlighted(&self, v: bool) -> Self {
    Block::from(self.clone())
      .title_style(if v { self.style().yellow() } else { self.style().fg(Color::Reset) })
      .border_style(if v { self.style().yellow() } else { self.style().fg(Color::Reset) })
  }
}

pub trait TextAreaTrait {
  fn clear(&mut self);
  fn set_text(&mut self, text: String);
  fn first_line_text(&mut self) -> String;
  fn highlight_border(&mut self, v: bool);
  fn highlight_text(&mut self, v: bool);
}

impl TextAreaTrait for TextArea<'_> {
  fn clear(&mut self) {
    self.move_cursor(tui_textarea::CursorMove::Head);
    self.delete_str(9999);
  }
  fn first_line_text(&mut self) -> String {
    self.lines()[0].clone()
  }
  fn set_text(&mut self, text: String) {
    self.clear();
    self.insert_str(text);
  }
  fn highlight_border(&mut self, v: bool) {
    self.set_block(
      Block::from(
        self
          .block()
          .unwrap()
          .clone()
          .border_style(if v { Style::new().yellow() } else { Style::new().reset() })
          .title_style(if v { Style::new().yellow() } else { Style::new().reset() })
      )
    )
  }
  fn highlight_text(&mut self, v: bool) {
    self.set_style(if v { self.style().yellow() } else { self.style().reset() });
  }
}
