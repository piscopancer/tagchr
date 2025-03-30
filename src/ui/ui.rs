use std::io::Stdout;
use bitflags::{ bitflags, Flags };
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use ratatui::{
  prelude::CrosstermBackend,
  style::{ Color, Style, Styled, Stylize },
  widgets::{ Block, BorderType, Borders, List, Paragraph, Table },
  Frame,
  Terminal,
};
use tui_textarea::TextArea;
use crate::app::app::{ App, State };
use super::{ home::{ self, screen::HomeScreen }, lyrics::screen::LyricsScreen };

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
  ChangeScreen(ui_enums::ScreenKind),
}

pub struct Ui {
  term: Terminal<CrosstermBackend<Stdout>>,
  pub screen: ui_enums::Screen,
}

impl Ui {
  pub fn new(state: Option<&State>) -> Self {
    Self {
      term: ratatui::init(),
      screen: ui_enums::Screen::Home(HomeScreen::new(home::screen::SelectableItem::Search, state)),
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
  fn navigate(&mut self, to: &mut ui_enums::ScreenKind, state: &mut State) {
    let from = &self.screen;
    match (from, to) {
      (ui_enums::Screen::Home(_), ui_enums::ScreenKind::Home) => {}
      (ui_enums::Screen::Lyrics(_), ui_enums::ScreenKind::Lyrics) => {}
      //
      (ui_enums::Screen::Home(home_screen), ui_enums::ScreenKind::Lyrics) => {
        let i = self.selected_song_index().unwrap();
        let lyrics = state.found_mp3_files[i].tags.lyrics.clone();
        self.screen = ui_enums::Screen::Lyrics(LyricsScreen::new(i, lyrics));
      }
      (ui_enums::Screen::Lyrics(lyrics_screen), ui_enums::ScreenKind::Home) => {
        state.found_mp3_files[lyrics_screen.index].tags.lyrics = lyrics_screen.lyrics.clone();
        self.screen = ui_enums::Screen::Home(
          HomeScreen::new(home::screen::SelectableItem::Table(lyrics_screen.index), Some(state))
        );
      }
    }
  }
  pub fn selected_song_index(&self) -> Option<usize> {
    match &self.screen {
      ui_enums::Screen::Home(home_screen) => {
        return match home_screen.selection {
          home::screen::SelectableItem::Table(i) => Some(i),
          home::screen::SelectableItem::Editor(i, _) => Some(i),
          _ => None,
        };
      }
      ui_enums::Screen::Lyrics(lyrics_screen) => Some(lyrics_screen.index),
    }
  }
  pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, _) => {
        state.running = false;
      }
      (code, modifiers) => {
        // TODO: distribute logic between screens, make them responsible for that
        match &mut self.screen {
          ui_enums::Screen::Home(screen) => {
            if let Some(cmd) = &mut screen.handle_key_event(key_event, state) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  self.navigate(screen, state);
                }
              }
            }
          }
          ui_enums::Screen::Lyrics(screen) => {
            if let Some(cmd) = &mut screen.handle_key_event(key_event, state) {
              match cmd {
                UiCommand::ChangeScreen(screen) => {
                  self.navigate(screen, state);
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

pub fn basic_text_area(title: String, text: Option<String>) -> TextArea<'static> {
  let mut input = TextArea::default();
  input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top(title));
  if let Some(text) = text {
    input.set_text(text);
  }
  input
}

// pub trait TableTrait {
//   fn created_highlighted(&mut self, prev_block: Block<'static>, v: bool) -> Self;
// }

// impl TableTrait for Table<'_> {
//   fn created_highlighted(&mut self, prev_block: Block<'static>, v: bool) -> Self {
//     self.clone().block(
//       Block::from(prev_block)
//         .title_style(if v { Style::new().yellow() } else { Style::new().reset() })
//         .border_style(if v { Style::new().yellow() } else { Style::new().reset() })
//     )
//   }
// }

bitflags! {
  #[derive(PartialEq)]
  struct WidgetStyle: u8 {
    const Selected = 0b00000001;
    const Disabled = 0b00000010;
  }
}

fn get_style(style: Option<WidgetStyle>) -> Style {
  if let Some(style) = style {
    match style {
      WidgetStyle::Selected => Style::new().yellow(),
      WidgetStyle::Disabled => Style::new().dark_gray(),
      WidgetStyle::Disabled | WidgetStyle::Selected => Style::new().dark_gray(),
      _ => Style::new(),
    }
  } else {
    Style::new()
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
}

// ---

pub trait SelectableWidget {
  fn focus(&mut self, v: bool);
  fn focused(&mut self, v: bool) -> Self;
}

pub trait WidgetWithEditableContent {
  fn highlight_content(&mut self, v: bool);
}

pub trait TagWidget: SelectableWidget + WidgetWithEditableContent {
  // Can add tag-specific methods here
  // fn tag_operation(&self) {
  //   self.highlight_border();
  //   self.highlight_content();
  // }
}

impl<T> TagWidget for T where T: SelectableWidget + WidgetWithEditableContent {}

impl SelectableWidget for TextArea<'_> {
  fn focus(&mut self, v: bool) {
    self.set_cursor_style(if v { Style::new().reversed() } else { Style::new() });
    self.set_block(Block::from(self.block().unwrap().clone().highlighted(v)))
  }
  fn focused(&mut self, v: bool) -> Self {
    self.focus(v);
    self.clone()
  }
}

impl WidgetWithEditableContent for TextArea<'_> {
  fn highlight_content(&mut self, v: bool) {
    self.set_style(if v { self.style().yellow() } else { self.style().reset() });
  }
}

#[derive(Clone)]
pub struct SelectableTable {
  init_block: Block<'static>,
  pub table: Table<'static>,
}

impl SelectableTable {
  pub fn new(table: Table<'static>, backup_block: Block<'static>) -> Self {
    Self { table, init_block: backup_block }
  }
}

impl SelectableWidget for SelectableTable {
  fn focus(&mut self, v: bool) {
    self.table = self.table.clone().block(self.init_block.highlighted(v));
  }
  fn focused(&mut self, v: bool) -> Self {
    self.focus(v);
    self.clone()
  }
}

#[derive(Clone)]
pub struct SelectableParagraph {
  init_block: Block<'static>,
  pub paragraph: Paragraph<'static>,
}

impl SelectableParagraph {
  pub fn new(paragraph: Paragraph<'static>, backup_block: Block<'static>) -> Self {
    Self { paragraph, init_block: backup_block }
  }
}

impl SelectableWidget for SelectableParagraph {
  fn focus(&mut self, v: bool) {
    self.paragraph = self.paragraph.clone().block(self.init_block.highlighted(v));
  }
  fn focused(&mut self, v: bool) -> Self {
    self.focus(v);
    self.clone()
  }
}

// TOGGLEABLE WIDGET
// can be enabled/disabled

trait ToggleableWidget {
  fn toggle(&mut self, v: bool);
}

impl ToggleableWidget for TextArea<'_> {
  fn toggle(&mut self, v: bool) {
    self.set_block(block);
  }
}
