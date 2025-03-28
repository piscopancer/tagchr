use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use id3::TagLike;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Rect },
  style::{ Style, Stylize },
  text::Line,
  widgets::{ block::Title, Block, BorderType, Cell, List, Paragraph, Row, Table, Widget },
  Frame,
};
use tui_textarea::TextArea;
use uuid::Uuid;
use crate::{
  app::app::{ App, EditableTag, EditableTagTrait, SongTags, State, TagState },
  ui::{
    lyrics::screen::LyricsScreen,
    ui::{ basic_text_area, ui_enums, BlockTrait, Screen, TableTrait, TextAreaTrait, UiCommand },
  },
};

#[derive(PartialEq, Debug)]
pub enum EditorSectionSelectable {
  TitleInput,
  ArtistInput,
  YearInput,
  Genre,
  Lyrics,
}

#[derive(PartialEq, Debug)]
pub enum HomeScreenSection {
  Search,
  Table(usize),
  Editor(usize, EditorSectionSelectable),
}

pub struct HomeScreen {
  selected_section: HomeScreenSection,
  search_input: TextArea<'static>,
  files_table: Table<'static>,
  title_input: TextArea<'static>,
  artist_input: TextArea<'static>,
  year_input: TextArea<'static>,
  genre_input: TextArea<'static>,
  lyrics_button: Paragraph<'static>,
}

pub const TABLE_BLOCK: Block<'static> = Block::bordered().border_type(BorderType::Rounded);

impl HomeScreen {
  pub fn new() -> Self {
    Self {
      selected_section: HomeScreenSection::Search,
      files_table: Table::default()
        .column_spacing(1)
        .widths([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
        .block(TABLE_BLOCK),
      search_input: basic_text_area("Search".into()),
      title_input: basic_text_area("Title".into()),
      artist_input: basic_text_area("Artist".into()),
      year_input: basic_text_area("Year".into()),
      genre_input: basic_text_area("Genre".into()),
      lyrics_button: Paragraph::new(
        vec![Line::default(), Line::from("Edit").centered(), Line::default()]
      ).block(Block::bordered().border_type(BorderType::Rounded).title_top("Lyrics")),
    }
  }
}

impl Screen for HomeScreen {
  fn draw(&mut self, frame: &mut Frame, state: &State) {
    let hor_l = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [sidebar_area, editor_area] = hor_l.areas(frame.area());
    let vert_l = Layout::vertical([
      Constraint::Length(1),
      Constraint::Length(3),
      Constraint::Fill(1),
    ]);
    let [debug_area, path_input_area, list_area] = vert_l.areas(sidebar_area);
    let [
      title_input_area,
      artist_input_area,
      year_input_area,
      genre_input_area,
      lyrics_button_area,
    ] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(5),
    ])
      .flex(Flex::Start)
      .areas(editor_area);

    self.search_input.highlight_border(self.selected_section == HomeScreenSection::Search);
    self.files_table = self.files_table.clone().rows({
      let rows = state.found_mp3_files
        .iter()
        .enumerate()
        .map(|(i, f)|
          Row::new(
            vec![
              Cell::new(f.name.clone()),
              Cell::new(f.path.clone().green().italic()),
              Cell::new(f.modified_date.clone().dark_gray())
            ]
          ).style(match &self.selected_section {
            HomeScreenSection::Table(i2) if *i2 == i => { Style::new().reversed() }
            HomeScreenSection::Editor(i2, ..) if *i2 == i => { Style::new().on_dark_gray() }
            _ => { Style::new() }
          })
        )
        .collect::<Vec<_>>();
      rows
    });
    self.files_table = self.files_table.created_highlighted(TABLE_BLOCK, match
      self.selected_section
    {
      HomeScreenSection::Table(_) => true,
      _ => false,
    });
    self.title_input.highlight_border(match self.selected_section {
      HomeScreenSection::Editor(_, EditorSectionSelectable::TitleInput) => true,
      _ => false,
    });

    self.artist_input.highlight_border(match self.selected_section {
      HomeScreenSection::Editor(_, EditorSectionSelectable::ArtistInput) => true,
      _ => false,
    });
    self.year_input.highlight_border(match self.selected_section {
      HomeScreenSection::Editor(_, EditorSectionSelectable::YearInput) => true,
      _ => false,
    });
    self.genre_input.highlight_border(match self.selected_section {
      HomeScreenSection::Editor(_, EditorSectionSelectable::Genre) => true,
      _ => false,
    });
    self.lyrics_button = self.lyrics_button.clone().block(
      Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Lyrics")
        .highlighted(match self.selected_section {
          HomeScreenSection::Editor(_, EditorSectionSelectable::Lyrics) => true,
          _ => false,
        })
    );
    frame.render_widget(&self.search_input, path_input_area);
    frame.render_widget(&self.files_table, list_area);
    frame.render_widget(&self.title_input, title_input_area);
    frame.render_widget(&self.artist_input, artist_input_area);
    frame.render_widget(&self.year_input, year_input_area);
    frame.render_widget(&self.genre_input, genre_input_area);
    frame.render_widget(&self.lyrics_button, lyrics_button_area);
    // debug
    let debug_p = Paragraph::new(
      vec![Line::from(format!("sec [{:?}] ", self.selected_section))]
    ).light_magenta();
    frame.render_widget(&debug_p, debug_area);
  }

  fn handle_key_event<'a>(
    &'a mut self,
    key_event: KeyEvent,
    state: &'a mut State
  ) -> Option<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Up, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Table(_) => {
            self.selected_section = HomeScreenSection::Search;
            self.title_input.clear();
            self.artist_input.clear();
            self.year_input.clear();
            self.genre_input.clear();
            self.title_input.highlight_text(false);
            self.artist_input.highlight_text(false);
            self.year_input.highlight_text(false);
            self.genre_input.highlight_text(false);
            None
          }
          HomeScreenSection::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSectionSelectable::TitleInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::Lyrics
                );
                None
              }
              EditorSectionSelectable::ArtistInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::TitleInput
                );
                None
              }
              EditorSectionSelectable::YearInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::ArtistInput
                );
                None
              }
              EditorSectionSelectable::Genre => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::YearInput
                );
                None
              }
              EditorSectionSelectable::Lyrics => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::Genre
                );
                None
              }
            }
          }
          _ => None,
        }
      }
      (KeyCode::Up, KeyModifiers::NONE) => {
        match self.selected_section {
          HomeScreenSection::Table(i) => {
            let new_i = if i > 0 { i - 1 } else { state.found_mp3_files.len() - 1 };
            self.selected_section = HomeScreenSection::Table(new_i);
            state.selected_song = {
              let path = state.found_mp3_files[new_i].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              state.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
            self.artist_input.set_text(
              state.selected_song.as_ref().unwrap().artist.original.clone().unwrap_or_default()
            );
            self.year_input.set_text(
              state.selected_song.as_ref().unwrap().year.original.clone().unwrap_or_default()
            );
            self.genre_input.set_text(
              state.selected_song.as_ref().unwrap().genre.original.clone().unwrap_or_default()
            );
            None
          }
          _ => None,
        }
      }
      (KeyCode::Right, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Table(i) => {
            self.selected_section = HomeScreenSection::Editor(
              *i,
              EditorSectionSelectable::TitleInput
            );
            None
          }

          _ => None,
        }
      }
      (KeyCode::Down, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Search => {
            self.selected_section = HomeScreenSection::Table(0);
            state.selected_song = {
              let path = state.found_mp3_files[0].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              state.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
            self.artist_input.set_text(
              state.selected_song.as_ref().unwrap().artist.original.clone().unwrap_or_default()
            );
            self.year_input.set_text(
              state.selected_song.as_ref().unwrap().year.original.clone().unwrap_or_default()
            );
            self.genre_input.set_text(
              state.selected_song.as_ref().unwrap().genre.original.clone().unwrap_or_default()
            );
            self.title_input.highlight_text(false);
            self.artist_input.highlight_text(false);
            self.year_input.highlight_text(false);
            self.genre_input.highlight_text(false);
            None
          }
          HomeScreenSection::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSectionSelectable::TitleInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::ArtistInput
                );
                None
              }
              EditorSectionSelectable::ArtistInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::YearInput
                );
                None
              }
              EditorSectionSelectable::YearInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::Genre
                );
                None
              }
              EditorSectionSelectable::Genre => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::Lyrics
                );
                None
              }
              EditorSectionSelectable::Lyrics => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::TitleInput
                );
                None
              }
            }
          }
          _ => None,
        }
      }
      (KeyCode::Down, KeyModifiers::NONE) => {
        match self.selected_section {
          HomeScreenSection::Table(i) => {
            let new_i = if i == state.found_mp3_files.len() - 1 { 0 } else { i + 1 };
            self.selected_section = HomeScreenSection::Table(new_i);
            state.selected_song = {
              let path = state.found_mp3_files[new_i].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              state.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
            self.artist_input.set_text(
              state.selected_song.as_ref().unwrap().artist.original.clone().unwrap_or_default()
            );
            self.year_input.set_text(
              state.selected_song.as_ref().unwrap().year.original.clone().unwrap_or_default()
            );
            self.genre_input.set_text(
              state.selected_song.as_ref().unwrap().genre.original.clone().unwrap_or_default()
            );
            self.title_input.highlight_text(false);
            self.artist_input.highlight_text(false);
            self.year_input.highlight_text(false);
            self.genre_input.highlight_text(false);
            None
          }
          _ => None,
        }
      }
      (KeyCode::Left, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Editor(i, _) => {
            self.selected_section = HomeScreenSection::Table(*i);
            None
          }
          _ => None,
        }
      }
      (KeyCode::Enter | KeyCode::Backspace | KeyCode::Tab, ..) if
        match &self.selected_section {
          | HomeScreenSection::Table(_)
          | HomeScreenSection::Editor(_, EditorSectionSelectable::Lyrics) => true,
          _ => false,
        }
      => {
        match &mut self.selected_section {
          HomeScreenSection::Table(i) => {
            let song_file = &state.found_mp3_files[*i];
            state.selected_song = Some(SongTags::new(song_file.path.clone()));
            self.selected_section = HomeScreenSection::Editor(
              *i,
              EditorSectionSelectable::TitleInput
            );
            None
          }
          HomeScreenSection::Editor(i, editor_section) => {
            match editor_section {
              EditorSectionSelectable::Lyrics => {
                Some(
                  UiCommand::ChangeScreen(
                    ui_enums::Screen::Lyrics(
                      LyricsScreen::new(state.selected_song.take().unwrap().lyrics)
                    )
                  )
                )
              }
              _ => None,
            }
          }
          _ => None,
        }
      }
      (key_code, modifiers) => {
        match &self.selected_section {
          HomeScreenSection::Search => {
            self.search_input.input(key_event);
            None
          }
          HomeScreenSection::Editor(_, editor_section) => {
            match editor_section {
              EditorSectionSelectable::TitleInput => {
                if self.title_input.input(key_event) {
                  if let Some(song) = &mut state.selected_song {
                    song.name.edit(self.title_input.first_line_text());
                    self.title_input.highlight_text(match song.name.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
                None
              }
              EditorSectionSelectable::ArtistInput => {
                if self.artist_input.input(key_event) {
                  if let Some(song) = &mut state.selected_song {
                    song.artist.edit(self.artist_input.first_line_text());
                    self.artist_input.highlight_text(match song.artist.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
                None
              }
              EditorSectionSelectable::YearInput => {
                if self.year_input.input(key_event) {
                  if let Some(song) = &mut state.selected_song {
                    song.year.edit(self.year_input.first_line_text());
                    self.year_input.highlight_text(match song.year.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
                None
              }
              EditorSectionSelectable::Genre => {
                if self.genre_input.input(key_event) {
                  if let Some(song) = &mut state.selected_song {
                    song.genre.edit(self.genre_input.first_line_text());
                    self.genre_input.highlight_text(match song.genre.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
                None
              }
              _ => None,
            }
          }
          _ => None,
        }
      }
      _ => None,
    }
  }
}
