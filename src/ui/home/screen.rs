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
  app::app::{ App, Editable, EditableState, LyricsEditableTag, SongTags, State },
  ui::{
    lyrics::screen::LyricsScreen,
    ui::{
      basic_text_area,
      ui_enums,
      BlockTrait,
      Screen,
      SelectableParagraph,
      SelectableTable,
      SelectableWidget,
      TextAreaTrait,
      Ui,
      UiCommand,
      WidgetWithEditableContent,
    },
  },
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EditorSelectableItem {
  TitleInput,
  ArtistInput,
  YearInput,
  GenreInput,
  LyricsButton,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SelectableItem {
  Search,
  Table(usize),
  Editor(usize, EditorSelectableItem),
}

pub struct HomeScreen {
  pub selection: SelectableItem,
  search_input: TextArea<'static>,
  files_table: SelectableTable,
  title_input: TextArea<'static>,
  artist_input: TextArea<'static>,
  year_input: TextArea<'static>,
  genre_input: TextArea<'static>,
  lyrics_button: SelectableParagraph,
}

impl HomeScreen {
  pub fn new(selection: SelectableItem, state: Option<&State>) -> Self {
    let song_tags = state
      .map(|s| {
        match &selection {
          SelectableItem::Table(i) | SelectableItem::Editor(i, _) =>
            Some(&s.found_mp3_files[*i].tags),
          _ => None,
        }
      })
      .flatten();
    Self {
      selection,
      files_table: {
        let b = Block::bordered().border_type(BorderType::Rounded);
        SelectableTable::new(
          Table::default()
            .column_spacing(1)
            .widths([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
            .block(b.clone()),
          b
        )
      },
      search_input: basic_text_area(
        "Search".into(),
        state.map(|s| s.search.clone())
      ).focused(selection == SelectableItem::Search),
      title_input: basic_text_area(
        "Title".into(),
        song_tags.map(|t| t.title.0.to_string())
      ).focused(match selection {
        SelectableItem::Editor(_, EditorSelectableItem::TitleInput) => true,
        _ => false,
      }),
      artist_input: basic_text_area(
        "Artist".into(),
        song_tags.map(|t| t.artist.0.to_string())
      ).focused(match selection {
        SelectableItem::Editor(_, EditorSelectableItem::ArtistInput) => true,
        _ => false,
      }),
      year_input: basic_text_area(
        "Year".into(),
        song_tags.map(|t| t.year.0.to_string())
      ).focused(match selection {
        SelectableItem::Editor(_, EditorSelectableItem::YearInput) => true,
        _ => false,
      }),
      genre_input: basic_text_area(
        "Genre".into(),
        song_tags.map(|t| t.genre.0.to_string())
      ).focused(match selection {
        SelectableItem::Editor(_, EditorSelectableItem::GenreInput) => true,
        _ => false,
      }),
      lyrics_button: {
        let b = Block::bordered().border_type(BorderType::Rounded).title_top("Lyrics");
        let p = SelectableParagraph::new(
          Paragraph::new(
            vec![Line::default(), Line::from("Edit").centered(), Line::default()]
          ).block(b.clone()),
          b
        );
        p
      },
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

    self.search_input.focus(self.selection == SelectableItem::Search);
    self.files_table.table = self.files_table.table.clone().rows({
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
          ).style(match &self.selection {
            SelectableItem::Table(i2) if *i2 == i => { Style::new().reversed() }
            SelectableItem::Editor(i2, ..) if *i2 == i => { Style::new().on_dark_gray() }
            _ => { Style::new() }
          })
        )
        .collect::<Vec<_>>();
      rows
    });
    self.files_table.focus(match self.selection {
      SelectableItem::Table(_) => true,
      _ => false,
    });
    self.title_input.focus(match self.selection {
      SelectableItem::Editor(_, EditorSelectableItem::TitleInput) => true,
      _ => false,
    });

    self.artist_input.focus(match self.selection {
      SelectableItem::Editor(_, EditorSelectableItem::ArtistInput) => true,
      _ => false,
    });
    self.year_input.focus(match self.selection {
      SelectableItem::Editor(_, EditorSelectableItem::YearInput) => true,
      _ => false,
    });
    self.genre_input.focus(match self.selection {
      SelectableItem::Editor(_, EditorSelectableItem::GenreInput) => true,
      _ => false,
    });
    self.lyrics_button.focus(match self.selection {
      SelectableItem::Editor(_, EditorSelectableItem::LyricsButton) => true,
      _ => false,
    });

    frame.render_widget(&self.search_input, path_input_area);
    frame.render_widget(&self.files_table.table, list_area);
    frame.render_widget(&self.title_input, title_input_area);
    frame.render_widget(&self.artist_input, artist_input_area);
    frame.render_widget(&self.year_input, year_input_area);
    frame.render_widget(&self.genre_input, genre_input_area);
    frame.render_widget(&self.lyrics_button.paragraph, lyrics_button_area);
    // debug
    let debug_p = Paragraph::new(
      vec![Line::from(format!("sec [{:?}] ", self.selection))]
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
        match &self.selection {
          SelectableItem::Table(_) => {
            self.selection = SelectableItem::Search;
            self.title_input.clear();
            self.artist_input.clear();
            self.year_input.clear();
            self.genre_input.clear();
            self.title_input.highlight_content(false);
            self.artist_input.highlight_content(false);
            self.year_input.highlight_content(false);
            self.genre_input.highlight_content(false);
            None
          }
          SelectableItem::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSelectableItem::TitleInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::LyricsButton);
                None
              }
              EditorSelectableItem::ArtistInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::TitleInput);
                None
              }
              EditorSelectableItem::YearInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::ArtistInput);
                None
              }
              EditorSelectableItem::GenreInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::YearInput);
                None
              }
              EditorSelectableItem::LyricsButton => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::GenreInput);
                None
              }
            }
          }
          _ => None,
        }
      }
      (KeyCode::Up, KeyModifiers::NONE) => {
        match self.selection {
          SelectableItem::Table(i) => {
            let new_i = if i > 0 { i - 1 } else { state.found_mp3_files.len() - 1 };
            self.selection = SelectableItem::Table(new_i);
            let tags = &state.found_mp3_files.get(new_i).unwrap().tags;
            self.title_input.set_text(tags.title.0.to_string());
            self.artist_input.set_text(tags.artist.0.to_string());
            self.year_input.set_text(tags.year.0.to_string());
            self.genre_input.set_text(tags.genre.0.to_string());
            None
          }
          _ => None,
        }
      }
      (KeyCode::Right, KeyModifiers::CONTROL) => {
        match &self.selection {
          SelectableItem::Table(i) => {
            self.selection = SelectableItem::Editor(*i, EditorSelectableItem::TitleInput);
            None
          }

          _ => None,
        }
      }
      (KeyCode::Down, KeyModifiers::CONTROL) => {
        match &self.selection {
          SelectableItem::Search => {
            self.selection = SelectableItem::Table(0);
            let tags = &state.found_mp3_files[0].tags;
            self.title_input.set_text(tags.title.0.to_string());
            self.artist_input.set_text(tags.artist.0.to_string());
            self.year_input.set_text(tags.year.0.to_string());
            self.genre_input.set_text(tags.genre.0.to_string());
            self.title_input.highlight_content(false);
            self.artist_input.highlight_content(false);
            self.year_input.highlight_content(false);
            self.genre_input.highlight_content(false);
            None
          }
          SelectableItem::Editor(i, editor_selection) => {
            match editor_selection {
              EditorSelectableItem::TitleInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::ArtistInput);
                None
              }
              EditorSelectableItem::ArtistInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::YearInput);
                None
              }
              EditorSelectableItem::YearInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::GenreInput);
                None
              }
              EditorSelectableItem::GenreInput => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::LyricsButton);
                None
              }
              EditorSelectableItem::LyricsButton => {
                self.selection = SelectableItem::Editor(*i, EditorSelectableItem::TitleInput);
                None
              }
            }
          }
          _ => None,
        }
      }
      (KeyCode::Down, KeyModifiers::NONE) => {
        match self.selection {
          SelectableItem::Table(i) => {
            let new_i = if i == state.found_mp3_files.len() - 1 { 0 } else { i + 1 };
            self.selection = SelectableItem::Table(new_i);
            let tags = &state.found_mp3_files[new_i].tags;
            self.title_input.set_text(tags.title.0.to_string());
            self.artist_input.set_text(tags.artist.0.to_string());
            self.year_input.set_text(tags.year.0.to_string());
            self.genre_input.set_text(tags.genre.0.to_string());
            self.title_input.highlight_content(false);
            self.artist_input.highlight_content(false);
            self.year_input.highlight_content(false);
            self.genre_input.highlight_content(false);
            None
          }
          _ => None,
        }
      }
      (KeyCode::Left, KeyModifiers::CONTROL) => {
        match &self.selection {
          SelectableItem::Editor(i, _) => {
            self.selection = SelectableItem::Table(*i);
            None
          }
          _ => None,
        }
      }
      (KeyCode::Enter | KeyCode::Backspace | KeyCode::Tab, ..) if
        match &self.selection {
          | SelectableItem::Table(_)
          | SelectableItem::Editor(_, EditorSelectableItem::LyricsButton) => true,
          _ => false,
        }
      => {
        match &mut self.selection {
          SelectableItem::Table(i) => {
            self.selection = SelectableItem::Editor(*i, EditorSelectableItem::TitleInput);
            None
          }
          SelectableItem::Editor(i, editor_section) => {
            match editor_section {
              EditorSelectableItem::LyricsButton => {
                Some(UiCommand::ChangeScreen(ui_enums::ScreenKind::Lyrics))
              }
              _ => None,
            }
          }
          _ => None,
        }
      }
      (key_code, modifiers) => {
        match &self.selection {
          SelectableItem::Search => {
            self.search_input.input(key_event);
            None
          }
          SelectableItem::Editor(i, editor_section) => {
            let tags = &mut state.found_mp3_files[*i].tags;
            match editor_section {
              EditorSelectableItem::TitleInput => {
                if self.title_input.input(key_event) {
                  tags.title.0.edit(self.title_input.first_line_text());
                  self.title_input.highlight_content(match tags.title.0.state {
                    EditableState::Changed(_) => true,
                    _ => false,
                  });
                }
                None
              }
              EditorSelectableItem::ArtistInput => {
                if self.artist_input.input(key_event) {
                  tags.artist.0.edit(self.artist_input.first_line_text());
                  self.artist_input.highlight_content(match tags.artist.0.state {
                    EditableState::Changed(_) => true,
                    _ => false,
                  });
                }
                None
              }
              EditorSelectableItem::YearInput => {
                if self.year_input.input(key_event) {
                  tags.year.0.edit(self.year_input.first_line_text());
                  self.year_input.highlight_content(match tags.year.0.state {
                    EditableState::Changed(_) => true,
                    _ => false,
                  });
                }
                None
              }
              EditorSelectableItem::GenreInput => {
                if self.genre_input.input(key_event) {
                  tags.genre.0.edit(self.genre_input.first_line_text());
                  self.genre_input.highlight_content(match tags.genre.0.state {
                    EditableState::Changed(_) => true,
                    _ => false,
                  });
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
  // fn navigate(&mut self, ui: &mut Ui, to: ui_enums::ScreenKind) {
  //   match to {
  //     ui_enums::ScreenKind::Home => {}
  //     //
  //     ui_enums::ScreenKind::Lyrics => {
  //       ui.screen = ui_enums::Screen::Lyrics(LyricsScreen::new(ui.selected_song_index().unwrap()));
  //     }
  //   }
  // }
}
