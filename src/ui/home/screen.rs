use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use id3::TagLike;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Rect },
  style::{ Style, Stylize },
  text::Line,
  widgets::{ Block, BorderType, Cell, List, Paragraph, Row, Table, Widget },
  Frame,
};
use tui_textarea::TextArea;
use uuid::Uuid;
use crate::{
  app::app::{ App, EditableTag, SongTags, TagState },
  ui::ui::{ basic_text_area, Screen, TableTrait, TextAreaTrait },
};

#[derive(PartialEq, Debug)]
pub enum EditorSectionSelectable {
  TitleInput,
  ArtistInput,
  YearInput,
}

#[derive(PartialEq, Debug)]
pub enum HomeScreenSection {
  Path,
  Table(usize),
  Editor(usize, EditorSectionSelectable),
}

pub struct HomeScreen {
  selected_section: HomeScreenSection,
  search_input: TextArea<'static>,
  title_input: TextArea<'static>,
  artist_input: TextArea<'static>,
  year_input: TextArea<'static>,
  files_table: Table<'static>,
}

pub const TABLE_BLOCK: Block<'static> = Block::bordered().border_type(BorderType::Rounded);

impl HomeScreen {
  pub fn new() -> Self {
    Self {
      selected_section: HomeScreenSection::Path,
      search_input: basic_text_area("Search".into()),
      title_input: basic_text_area("Title".into()),
      artist_input: basic_text_area("Artist".into()),
      year_input: basic_text_area("Year".into()),
      files_table: Table::default()
        .column_spacing(1)
        .widths([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
        .block(TABLE_BLOCK),
    }
  }
}

impl Screen for HomeScreen {
  fn draw(&mut self, frame: &mut Frame, app: &App) {
    let hor_l = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [sidebar_area, editor_area] = hor_l.areas(frame.area());
    let vert_l = Layout::vertical([
      Constraint::Length(1),
      Constraint::Length(3),
      Constraint::Fill(1),
    ]);
    let [debug_area, path_input_area, list_area] = vert_l.areas(sidebar_area);
    let [song_name_input_area, song_artist_input_area, song_year_input_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
    ])
      .flex(Flex::Start)
      .areas(editor_area);

    self.search_input.highlight_border(self.selected_section == HomeScreenSection::Path);
    self.files_table = self.files_table.clone().rows({
      let rows = app.found_mp3_files
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

    frame.render_widget(&self.search_input, path_input_area);
    frame.render_widget(&self.files_table, list_area);
    frame.render_widget(&self.title_input, song_name_input_area);
    frame.render_widget(&self.artist_input, song_artist_input_area);
    frame.render_widget(&self.year_input, song_year_input_area);
    // deubug
    let debug_p = Paragraph::new(
      vec![Line::from(format!("sec [{:?}] ", self.selected_section))]
    ).light_magenta();
    frame.render_widget(&debug_p, debug_area);
  }

  fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Up, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Table(_) => {
            self.selected_section = HomeScreenSection::Path;
          }
          HomeScreenSection::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSectionSelectable::TitleInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::YearInput
                );
              }
              EditorSectionSelectable::ArtistInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::TitleInput
                );
              }
              EditorSectionSelectable::YearInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::ArtistInput
                );
              }
            }
          }
          _ => {}
        }
      }
      (KeyCode::Up, KeyModifiers::NONE) => {
        match self.selected_section {
          HomeScreenSection::Table(i) => {
            let new_i = if i > 0 { i - 1 } else { app.found_mp3_files.len() - 1 };
            self.selected_section = HomeScreenSection::Table(new_i);
            app.selected_song = {
              let path = app.found_mp3_files[new_i].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              app.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
          }
          _ => {}
        }
      }
      (KeyCode::Right, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Table(i) => {
            self.selected_section = HomeScreenSection::Editor(
              *i,
              EditorSectionSelectable::TitleInput
            );
          }
          _ => {}
        }
      }
      (KeyCode::Down, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Path => {
            self.selected_section = HomeScreenSection::Table(0);
            app.selected_song = {
              let path = app.found_mp3_files[0].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              app.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
          }
          HomeScreenSection::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSectionSelectable::TitleInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::ArtistInput
                );
              }
              EditorSectionSelectable::ArtistInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::YearInput
                );
              }
              EditorSectionSelectable::YearInput => {
                self.selected_section = HomeScreenSection::Editor(
                  *i,
                  EditorSectionSelectable::TitleInput
                );
              }
            }
          }
          _ => {}
        }
      }
      (KeyCode::Down, KeyModifiers::NONE) => {
        match self.selected_section {
          HomeScreenSection::Table(i) => {
            let new_i = if i == app.found_mp3_files.len() - 1 { 0 } else { i + 1 };
            self.selected_section = HomeScreenSection::Table(new_i);
            app.selected_song = {
              let path = app.found_mp3_files[new_i].path.clone();
              Some(SongTags::new(path))
            };
            self.title_input.set_text(
              app.selected_song.as_ref().unwrap().name.original.clone().unwrap_or_default()
            );
          }
          _ => {}
        }
      }
      (KeyCode::Left, KeyModifiers::CONTROL) => {
        match &self.selected_section {
          HomeScreenSection::Editor(..) => {
            self.selected_section = HomeScreenSection::Path;
          }
          _ => {}
        }
      }
      (KeyCode::Enter | KeyCode::Backspace | KeyCode::Tab, ..) if
        match &self.selected_section {
          HomeScreenSection::Table(_) => true,
          _ => false,
        }
      => {
        match self.selected_section {
          HomeScreenSection::Table(i) => {
            let song_file = &app.found_mp3_files[i];
            app.selected_song = Some(SongTags::new(song_file.path.clone()));
            self.selected_section = HomeScreenSection::Editor(
              i,
              EditorSectionSelectable::TitleInput
            );
          }
          _ => {}
        }
      }
      (key_code, modifiers) => {
        match &self.selected_section {
          HomeScreenSection::Path => {
            self.search_input.input(key_event);
          }
          HomeScreenSection::Editor(_, editor_section) => {
            match editor_section {
              EditorSectionSelectable::TitleInput => {
                if self.title_input.input(key_event) {
                  if let Some(song) = &mut app.selected_song {
                    song.name.edit(self.title_input.first_line_text());
                    self.title_input.highlight_text(match song.name.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
              }
              EditorSectionSelectable::ArtistInput => {
                if self.artist_input.input(key_event) {
                  if let Some(song) = &mut app.selected_song {
                    song.artist.edit(self.artist_input.first_line_text());
                    self.artist_input.highlight_text(match song.artist.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
              }
              EditorSectionSelectable::YearInput => {
                if self.year_input.input(key_event) {
                  if let Some(song) = &mut app.selected_song {
                    song.year.edit(self.year_input.first_line_text());
                    self.year_input.highlight_text(match song.year.state {
                      TagState::Changed(_) => true,
                      _ => false,
                    });
                  }
                }
              }
            }
          }
          _ => {}
        }
      }
    }
  }
}
