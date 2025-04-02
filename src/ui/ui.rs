use std::io::Stdout;
use bitflags::{ bitflags, bitflags_match, Flags };
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use ratatui::{
  prelude::CrosstermBackend,
  style::{ Color, Style, Styled, Stylize },
  widgets::{ Block, BorderType, Borders, List, Paragraph, Table },
  Frame,
  Terminal,
};
use tui_textarea::TextArea;
use crate::app::{ app::{ App, State }, tag::SongTags };
use super::{
  modal::Modals,
  screens::{ home::{ self, screen::HomeScreen }, lyrics::screen::LyricsScreen },
};

pub mod ui_enums {
  use kinded::Kinded;
  use crate::ui::screens::{ home::screen::HomeScreen, lyrics::screen::LyricsScreen };

  #[derive(Kinded)]
  pub enum Screen {
    Home(HomeScreen),
    Lyrics(LyricsScreen),
  }
}

pub enum UiCommand {
  Navigate(ui_enums::ScreenKind),
}

pub struct Ui {
  term: Terminal<CrosstermBackend<Stdout>>,
  pub modals: Modals,
  pub screen: ui_enums::Screen,
}

impl Ui {
  pub fn new() -> Self {
    Self {
      term: ratatui::init(),
      modals: Modals::new(),
      screen: ui_enums::Screen::Home(HomeScreen::new(home::screen::Focusable::Search)),
    }
  }
  pub fn draw(&mut self, state: &mut State) {
    self.term.draw(|frame| {
      match &mut self.screen {
        // TODO: impl Widget for screens even though they are just holders for actual widgets? may not make sense tbh
        ui_enums::Screen::Home(screen) => {
          screen.draw(frame, state);
        }
        ui_enums::Screen::Lyrics(screen) => {
          screen.draw(frame, state);
        }
      }
      // draw modals after screens bcs they need to sit on top
      for modal in self.modals.iter() {
        frame.render_widget(modal, frame.area());
      }
    });
  }
  fn navigate(&mut self, to: ui_enums::ScreenKind, state: &mut State) {
    let from = &self.screen;
    match (from, to) {
      (ui_enums::Screen::Home(_), ui_enums::ScreenKind::Home) => {}
      (ui_enums::Screen::Lyrics(_), ui_enums::ScreenKind::Lyrics) => {}
      //
      (ui_enums::Screen::Home(home_screen), ui_enums::ScreenKind::Lyrics) => {
        let i = self.selected_song_index().unwrap();
        let lyrics = state.searched_mp3_files[i].tags.lyrics.clone();
        self.screen = ui_enums::Screen::Lyrics(LyricsScreen::new(i, lyrics));
      }
      (ui_enums::Screen::Lyrics(lyrics_screen), ui_enums::ScreenKind::Home) => {
        state.searched_mp3_files[lyrics_screen.index].tags.lyrics = lyrics_screen.lyrics.clone();
        self.screen = ui_enums::Screen::Home(
          HomeScreen::new(
            home::screen::Focusable::Editor(
              lyrics_screen.index,
              home::screen::EditorFocusable::LyricsButton
            )
          )
        );
      }
    }
  }
  pub fn selected_song_index(&self) -> Option<usize> {
    match &self.screen {
      ui_enums::Screen::Home(home_screen) => {
        return match home_screen.focused_el {
          home::screen::Focusable::Table(i) => Some(i),
          home::screen::Focusable::Editor(i, _) => Some(i),
          _ => None,
        };
      }
      ui_enums::Screen::Lyrics(lyrics_screen) => Some(lyrics_screen.index),
    }
  }
  fn handle_command(&mut self, cmd: UiCommand, state: &mut State) {
    match cmd {
      UiCommand::Navigate(screen) => {
        self.navigate(screen, state);
      }
    }
  }
  pub fn song_tags<'a>(&'a self, state: &'a State) -> Option<&'a SongTags> {
    self.selected_song_index().map(|i| &state.searched_mp3_files[i].tags)
  }
  pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
    match (key_event.code, key_event.modifiers) {
      (code, modifiers) => {
        // TODO: popups/dialogues handling here
        // end iter if
        if let Some(modal) = self.modals.last() {
          // modal.handle_key_event(key_event, state);
        }
        match &mut self.screen {
          ui_enums::Screen::Home(screen) => {
            if let Some(cmd) = screen.handle_key_event(key_event, state) {
              self.handle_command(cmd, state);
            }
          }
          ui_enums::Screen::Lyrics(screen) => {
            if let Some(cmd) = screen.handle_key_event(key_event, state) {
              self.handle_command(cmd, state);
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

bitflags! {
  #[derive(PartialEq, Clone, Copy)]
  pub struct WidgetState: u8 {
    const Enabled = 1;
    const Highlighted = 1 << 1;
  }
}

pub trait StringTrait {
  fn to_single_line(&mut self) -> Self;
}

impl StringTrait for String {
  fn to_single_line(&mut self) -> Self {
    self.replace("\n", " ")
  }
}

impl From<WidgetState> for Style {
  fn from(state: WidgetState) -> Self {
    if state.is_empty() || !state.contains(WidgetState::Enabled) {
      Style::new().dark_gray()
    } else if state.contains(WidgetState::Highlighted) {
      Style::new().yellow()
    } else {
      Style::new()
    }
  }
}
