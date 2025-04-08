use std::{ io::Stdout, sync::mpsc::Sender };
use bitflags::{ bitflags, bitflags_match, Flags };
use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  prelude::CrosstermBackend,
  style::{ Color, Style, Styled, Stylize },
  widgets::{ Block, BorderType, Borders, List, Paragraph, Table },
  Frame,
  Terminal,
};
use tui_textarea::TextArea;
use crate::app::{ app::{ App, Command }, state::State, tag::SongTags };
use super::{
  modals::modal::{ enums::Modal, Modals },
  screens::{ home::{ self, screen::HomeScreen }, lyrics::screen::LyricsScreen },
};

pub trait StateDependentWidget {
  fn render_from_state(&self, area: Rect, buf: &mut Buffer, state: &State, ui_state: &UiState)
    where Self: Sized;
}

pub trait InputHandler {
  fn handle_input(
    &self,
    state: &State,
    ui_state: &UiState,
    event: Event,
    sender: Sender<Command>
  ) -> bool;
}

pub mod ui_enums {
  use kinded::Kinded;
  use crate::ui::screens::{ home::screen::HomeScreen, lyrics::screen::LyricsScreen };

  #[derive(Kinded)]
  pub enum Screen {
    Home(HomeScreen),
    Lyrics(LyricsScreen),
  }
}

pub struct UiState {
  pub modals: Modals,
  pub screen: ui_enums::Screen,
}

pub struct Ui {
  term: Terminal<CrosstermBackend<Stdout>>,
  pub state: UiState,
}

impl Ui {
  pub fn new() -> Self {
    Self {
      term: ratatui::init(),
      state: UiState {
        modals: Modals::new(),
        screen: ui_enums::Screen::Home(HomeScreen::new(home::screen::Focusable::Search, None)),
      },
    }
  }
  pub fn handle_input(&self, state: &State, event: Event, sender: Sender<Command>) {
    match event {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        if let Some(modal) = self.state.modals.last() {
          modal.handle_input(state, &self.state, event, sender);
          return;
        }
        match &self.state.screen {
          ui_enums::Screen::Home(screen) => {
            screen.handle_input(state, &self.state, event, sender);
          }
          ui_enums::Screen::Lyrics(screen) => {
            screen.handle_input(state, &self.state, event, sender);
          }
        }
      }
      _ => {}
    }
  }
  pub fn render(&mut self, state: &State) {
    self.term.draw(|frame| {
      match &self.state.screen {
        ui_enums::Screen::Home(screen) => {
          screen.render_from_state(frame.area(), frame.buffer_mut(), state, &self.state);
        }
        ui_enums::Screen::Lyrics(screen) => {
          screen.render_from_state(frame.area(), frame.buffer_mut(), state, &self.state);
        }
      }
      for modal in self.state.modals.iter() {
        modal.render_ref(frame.area(), frame.buffer_mut());
      }
    });
  }
  // fn navigate(&mut self, to: ui_enums::ScreenKind, state: &mut State) {
  //   let from = &self.screen;
  //   match (from, to) {
  //     (ui_enums::Screen::Home(_), ui_enums::ScreenKind::Home) => {}
  //     (ui_enums::Screen::Lyrics(_), ui_enums::ScreenKind::Lyrics) => {}
  //     //
  //     (ui_enums::Screen::Home(home_screen), ui_enums::ScreenKind::Lyrics) => {
  //       let i = self.selected_song_index().unwrap();
  //       let lyrics = state.searched_mp3_files[i].tags.lyrics.clone();
  //       self.screen = ui_enums::Screen::Lyrics(LyricsScreen::new(i, lyrics));
  //     }
  //     (ui_enums::Screen::Lyrics(lyrics_screen), ui_enums::ScreenKind::Home) => {
  //       let mut tags = &mut state.searched_mp3_files[lyrics_screen.index].tags;
  //       tags.lyrics = lyrics_screen.lyrics.clone();
  //       self.screen = ui_enums::Screen::Home(
  //         HomeScreen::new(
  //           home::screen::Focusable::Editor(
  //             lyrics_screen.index,
  //             home::screen::EditorFocusable::LyricsButton
  //           ),
  //           Some(tags)
  //         )
  //       );
  //     }
  //   }
  // }
  pub fn selected_song_index(&self) -> Option<usize> {
    match &self.state.screen {
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
  pub fn song_tags<'a>(&'a self, state: &'a State) -> Option<&'a SongTags> {
    self.selected_song_index().map(|i| &state.searched_mp3_files[i].tags)
  }
}

// pub trait Screen {
//   fn draw(&mut self, frame: &mut Frame, state: &State);
//   fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Vec<UiCommand>;
// }

// bitflags! {
//   #[derive(PartialEq, Clone, Copy)]
//   pub struct WidgetFlags: u8 {
//     const Enabled = 1;
//     const Highlighted = 1 << 1;
//     const Valid = 1 << 2;
//   }
// }

#[derive(Copy, Clone)]
pub struct StyleFlags {
  pub enabled: bool,
  pub highlighted: bool,
  pub valid: bool,
}

pub trait StringTrait {
  fn to_single_line(&mut self) -> Self;
}

impl StringTrait for String {
  fn to_single_line(&mut self) -> Self {
    for pattern in ["\n", "\r"].iter() {
      self.replace(pattern, " ");
    }
    self.clone()
  }
}

impl From<StyleFlags> for Style {
  fn from(f: StyleFlags) -> Self {
    if !f.enabled {
      Style::new().dark_gray()
    } else if !f.valid {
      Style::new().red()
    } else if f.highlighted {
      Style::new().yellow()
    } else {
      Style::new()
    }
  }
}
