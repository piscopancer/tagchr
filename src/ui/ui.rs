use std::io::Stdout;
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use ratatui::{
  prelude::CrosstermBackend,
  style::{ Color, Style, Styled, Stylize },
  widgets::{ Block, BorderType, List, Table },
  Frame,
  Terminal,
};
use tui_textarea::TextArea;
use crate::app::app::{ App, State };
use super::{ home::screen::HomeScreen, lyrics::screen::LyricsScreen };

pub mod ui_enums {
  use crate::ui::{ home::screen::HomeScreen, lyrics::screen::LyricsScreen };
  use kinded::Kinded;

  #[derive(Kinded)]
  pub enum Screen {
    Home(HomeScreen),
    Lyrics(LyricsScreen),
  }
}

pub enum UiCommand {
  ChangeScreen(ui_enums::Screen),
}

pub struct Ui {
  term: Terminal<CrosstermBackend<Stdout>>,
  screen: ui_enums::Screen,
}

impl Ui {
  pub fn new() -> Self {
    Self {
      term: ratatui::init(),
      screen: ui_enums::Screen::Home(HomeScreen::new()),
    }
  }
  pub fn draw(&mut self, state: &mut State) {
    self.term
      .draw(|frame| {
        match &mut self.screen {
          ui_enums::Screen::Home(screen) => {
            screen.draw(frame, state);
          }
          ui_enums::Screen::Lyrics(screen) => {
            screen.draw(frame, state);
          }
        }
      })
      .map_err(|_| {
        state.running = false;
      });
  }
  pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, _) => {
        state.running = false;
      }
      (code, modifiers) => {
        match &mut self.screen {
          ui_enums::Screen::Home(screen) => {
            if let Some(cmd) = screen.handle_key_event(key_event, state) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  // self.screen = screen;
                }
              }
            }
          }
          ui_enums::Screen::Lyrics(screen) => {
            if let Some(cmd) = screen.handle_key_event(key_event, state) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  // self.screen = screen;
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
  fn draw(&mut self, frame: &mut Frame, state: &State);
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Option<UiCommand>;
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
