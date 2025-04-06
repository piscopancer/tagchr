use crate::{
  app::{ app::{ App, State }, tag::SongTags },
  info::{ PROJECT_DESC, PROJECT_NAME },
  ui::{
    block::BlockTrait,
    lyrics::screen::LyricsScreen,
    modals::modal,
    shortcut::Shortcut,
    text_area::TextAreaTrait,
    ui::{ ui_enums, Screen, Ui },
    widget::{ FocusableWidget, WidgetWithEditableContent },
    StringTrait,
    UiCommand,
    WidgetState,
  },
};
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use id3::TagLike;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Rect },
  style::{ Color, Style, Styled, Stylize },
  text::{ Line, Span },
  widgets::{ block::Title, Block, BorderType, Cell, List, Paragraph, Row, Table, Widget },
  Frame,
};
use tui_textarea::{ CursorMove, TextArea };
use uuid::Uuid;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EditorFocusable {
  TitleInput,
  ArtistInput,
  YearInput,
  GenreInput,
  LyricsButton,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Focusable {
  Search,
  Table(usize),
  Editor(usize, EditorFocusable),
}

pub struct HomeScreen {
  pub focused_el: Focusable,
  search_input: TextArea<'static>,
  title_input: TextArea<'static>,
  artist_input: TextArea<'static>,
  year_input: TextArea<'static>,
  genre_input: TextArea<'static>,
}

impl HomeScreen {
  pub fn new(selection: Focusable, song_tags: Option<&SongTags>) -> Self {
    Self {
      focused_el: selection,
      search_input: TextArea::custom(),
      title_input: {
        let mut input = TextArea::custom();
        input.set_text(song_tags.map(|t| t.title.0.to_string()).unwrap_or_default());
        input
      },
      artist_input: {
        let mut input = TextArea::custom();
        input.set_text(song_tags.map(|t| t.artist.0.to_string()).unwrap_or_default());
        input
      },
      genre_input: {
        let mut input = TextArea::custom();
        input.set_text(song_tags.map(|t| t.genre.0.to_string()).unwrap_or_default());
        input
      },
      year_input: {
        let mut input = TextArea::custom();
        input.set_text(song_tags.map(|t| t.year.0.to_string()).unwrap_or_default());
        input
      },
    }
  }
}

impl Screen for HomeScreen {
  fn draw(&mut self, frame: &mut Frame, state: &State) {
    let github_shortcut = Shortcut::new("Ctrl+G", "Github", Color::Gray);
    let help_shortcut = Shortcut::new("Ctrl+H", "Help", Color::Gray);
    let save_shortcut = Shortcut::new("Ctrl+S", "Save", Color::Yellow);

    let [header_area, main_area, footer_area] = Layout::vertical([
      Constraint::Length(1),
      Constraint::Fill(1),
      Constraint::Length(1),
    ]).areas(frame.area());
    let footer_area = footer_area.inner(Margin::new(1, 0));
    let hor_l = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [sidebar_area, editor_area] = hor_l.areas(main_area);
    let [
      //
      // debug_area,
      search_area,
      table_area,
    ] = Layout::vertical([
      //
      // Constraint::Length(1),
      Constraint::Length(3),
      Constraint::Fill(1),
    ]).areas(sidebar_area);
    let [table_title_file_area, table_title_path_area, table_title_mod_area] = Layout::horizontal([
      Constraint::Fill(1),
      Constraint::Fill(1),
      Constraint::Fill(1),
    ])
      .horizontal_margin(1)
      .spacing(1)
      .areas(table_area);
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

    self.search_input.toggle_cursor(self.focused_el == Focusable::Search);
    self.search_input.set_block(
      Block::bordered()
        .border_type(BorderType::Rounded)
        .title_top("Search")
        .state_styled({
          let mut ws = WidgetState::empty();
          ws.insert(WidgetState::Enabled);
          ws.set(WidgetState::Highlighted, self.focused_el == Focusable::Search);
          ws
        })
    );

    // TODO: combine with part below
    let mut files_table = {
      Table::default()
        .column_spacing(1)
        .widths([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
        .block(
          Block::bordered()
            .border_type(BorderType::Rounded)
            .state_styled({
              let mut ws = WidgetState::empty();
              ws.insert(WidgetState::Enabled);
              ws.set(WidgetState::Highlighted, match self.focused_el {
                Focusable::Table(..) => true,
                _ => false,
              });
              ws
            })
        )
    };
    files_table = files_table.rows({
      let rows = state.searched_mp3_files
        .iter()
        .enumerate()
        .map(|(i, f)| {
          Row::new(
            vec![
              Cell::new(f.name.clone()),
              Cell::new(f.path.clone().green().italic()),
              Cell::new(f.modified_date.clone().dark_gray())
            ]
          ).style(match &self.focused_el {
            Focusable::Table(i2) if *i2 == i => Style::new().reversed(),
            Focusable::Editor(i2, ..) if *i2 == i => Style::new().on_dark_gray(),
            _ => Style::new(),
          })
        })
        .collect::<Vec<_>>();
      rows
    });

    let table_title_file = Line::from(" MP3 file ").centered();
    let table_title_path = Line::from(" Path ").centered();
    let table_title_mod = Line::from(" Last mod ").centered();

    let rerender_tag_input = |
      input: &mut TextArea<'_>,
      title: &'static str,
      text: String,
      ws_0_enabled: bool,
      ws_0_hl: bool,
      ws_1_enabled: bool,
      ws_1_hl: bool
    | {
      let ws = {
        let mut ws = WidgetState::empty();
        ws.set(WidgetState::Enabled, ws_0_enabled);
        ws.set(WidgetState::Highlighted, ws_0_hl);
        ws
      };
      input.set_block(
        Block::bordered().border_type(BorderType::Rounded).title_top(title).state_styled(ws)
      );
      input.toggle_cursor(ws_0_hl);
      input.set_style(
        Style::from({
          let mut ws = WidgetState::empty();
          ws.set(WidgetState::Enabled, ws_1_enabled);
          ws.set(WidgetState::Highlighted, ws_1_hl);
          ws
        })
      );
    };

    let song_tags = match &self.focused_el {
      Focusable::Table(i) | Focusable::Editor(i, _) => Some(&state.searched_mp3_files[*i].tags),
      _ => None,
    };

    rerender_tag_input(
      &mut self.title_input,
      "Title",
      song_tags.map(|t| t.title.0.to_string()).unwrap_or_default(),
      song_tags.is_some(),
      match self.focused_el {
        Focusable::Editor(_, ed_f) =>
          match ed_f {
            EditorFocusable::TitleInput => true,
            _ => false,
          }
        _ => false,
      },
      song_tags.is_some(),
      song_tags.map(|t| t.title.0.edited()).unwrap_or_default()
    );

    rerender_tag_input(
      &mut self.artist_input,
      "Artist",
      song_tags.map(|t| t.artist.0.to_string()).unwrap_or_default(),
      song_tags.is_some(),
      match self.focused_el {
        Focusable::Editor(_, ed_f) =>
          match ed_f {
            EditorFocusable::ArtistInput => true,
            _ => false,
          }
        _ => false,
      },
      song_tags.is_some(),
      song_tags.map(|t| t.artist.0.edited()).unwrap_or_default()
    );

    rerender_tag_input(
      &mut self.genre_input,
      "Genre",
      song_tags.map(|t| t.genre.0.to_string()).unwrap_or_default(),
      song_tags.is_some(),
      match self.focused_el {
        Focusable::Editor(_, ed_f) =>
          match ed_f {
            EditorFocusable::GenreInput => true,
            _ => false,
          }
        _ => false,
      },
      song_tags.is_some(),
      song_tags.map(|t| t.genre.0.edited()).unwrap_or_default()
    );

    rerender_tag_input(
      &mut self.year_input,
      "Year",
      song_tags.map(|t| t.year.0.to_string()).unwrap_or_default(),
      song_tags.is_some(),
      match self.focused_el {
        Focusable::Editor(_, ed_f) =>
          match ed_f {
            EditorFocusable::YearInput => true,
            _ => false,
          }
        _ => false,
      },
      song_tags.is_some(),
      song_tags.map(|t| t.year.0.edited()).unwrap_or_default()
    );

    let mut lyrics_button = Paragraph::new(
      vec![
        Line::default(),
        Line::from("Edit").style(
          Style::from({
            let mut ws = WidgetState::empty();
            ws.set(WidgetState::Enabled, song_tags.is_some());
            ws.set(
              WidgetState::Highlighted,
              song_tags
                .map(
                  |t| (t.lyrics.lang.edited() || t.lyrics.desc.edited() || t.lyrics.text.edited())
                )
                .unwrap_or_default()
            );
            ws
          })
        ),
        Line::default()
      ]
    )
      .block(
        Block::bordered()
          .border_type(BorderType::Rounded)
          .title_top("Lyrics")
          .state_styled({
            let mut ws = WidgetState::empty();
            ws.set(WidgetState::Enabled, song_tags.is_some());
            ws.set(WidgetState::Highlighted, match self.focused_el {
              Focusable::Editor(_, ed_f) =>
                match ed_f {
                  EditorFocusable::LyricsButton => true,
                  _ => false,
                }
              _ => false,
            });
            ws
          })
      )
      .centered();

    // let debug_p = Paragraph::new(
    //   vec![Line::from(format!("sec [{:?}] ", self.focused_el))]
    // ).light_magenta();
    let header_line = Line::from(
      Vec::from([
        Span::from(": ").dark_gray(),
        Span::from(PROJECT_NAME),
        Span::from(" :").dark_gray(),
      ])
    );

    let footer_line = Line::from(
      Vec::from([
        if song_tags.is_some_and(|t| t.edited()) { save_shortcut.to_spans() } else { Vec::new() },
        if song_tags.is_some_and(|t| t.edited()) {
          Vec::from([Span::from(" :: ").dark_gray()])
        } else {
          Vec::new()
        },
        help_shortcut.to_spans(),
        Vec::from([Span::from(" :: ").dark_gray()]),
        github_shortcut.to_spans(),
      ]).concat()
    ).right_aligned();

    frame.render_widget(&header_line, header_area);
    frame.render_widget(&self.search_input, search_area);
    frame.render_widget(&files_table, table_area);
    frame.render_widget(&table_title_file, table_title_file_area);
    frame.render_widget(&table_title_path, table_title_path_area);
    frame.render_widget(&table_title_mod, table_title_mod_area);
    frame.render_widget(&self.title_input, title_input_area);
    frame.render_widget(&self.artist_input, artist_input_area);
    frame.render_widget(&self.year_input, year_input_area);
    frame.render_widget(&self.genre_input, genre_input_area);
    frame.render_widget(&lyrics_button, lyrics_button_area);
    frame.render_widget(&footer_line, footer_area);
  }
  fn handle_key_event<'a>(
    &'a mut self,
    key_event: KeyEvent,
    state: &'a mut State
  ) -> Vec<UiCommand> {
    let total_searched_mp3_files = state.searched_mp3_files.len();
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, _) => {
        state.running = false;
      }
      (KeyCode::PageUp, _) | (KeyCode::Up, KeyModifiers::CONTROL) =>
        match self.focused_el {
          Focusable::Table(_) => {
            self.focused_el = Focusable::Search;
            update_inputs(self, state);
          }
          Focusable::Editor(i, editor_selection) =>
            match editor_selection {
              EditorFocusable::TitleInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::LyricsButton);
              }
              EditorFocusable::ArtistInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::TitleInput);
              }
              EditorFocusable::YearInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::ArtistInput);
              }
              EditorFocusable::GenreInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::YearInput);
              }
              EditorFocusable::LyricsButton => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::GenreInput);
              }
            }
          Focusable::Search => {}
        }
      (KeyCode::Up, KeyModifiers::NONE) => {
        match self.focused_el {
          Focusable::Table(i) => {
            let new_i = if i > 0 { i - 1 } else { total_searched_mp3_files - 1 };
            self.focused_el = Focusable::Table(new_i);
            update_inputs(self, state);
          }
          _ => {}
        }
      }
      (KeyCode::End, _) | (KeyCode::Right, KeyModifiers::CONTROL) =>
        match self.focused_el {
          Focusable::Table(i) => {
            self.focused_el = Focusable::Editor(i, EditorFocusable::TitleInput);
          }
          Focusable::Editor(i, editor_section) =>
            match editor_section {
              EditorFocusable::LyricsButton => {
                return Vec::from([UiCommand::ChangeScreen(ui_enums::ScreenKind::Lyrics)]);
              }
              _ => {}
            }
          _ => {}
        }
      (KeyCode::PageDown, _) | (KeyCode::Down, KeyModifiers::CONTROL) =>
        match self.focused_el {
          Focusable::Search if total_searched_mp3_files > 0 => {
            self.focused_el = Focusable::Table(0);
            update_inputs(self, state);
          }
          Focusable::Editor(i, editor_selection) =>
            match editor_selection {
              EditorFocusable::TitleInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::ArtistInput);
              }
              EditorFocusable::ArtistInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::YearInput);
              }
              EditorFocusable::YearInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::GenreInput);
              }
              EditorFocusable::GenreInput => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::LyricsButton);
              }
              EditorFocusable::LyricsButton => {
                self.focused_el = Focusable::Editor(i, EditorFocusable::TitleInput);
              }
            }
          _ => {}
        }
      (KeyCode::Down, KeyModifiers::NONE) =>
        match self.focused_el {
          Focusable::Table(i) => {
            let new_i = if i == total_searched_mp3_files - 1 { 0 } else { i + 1 };
            self.focused_el = Focusable::Table(new_i);
            update_inputs(self, state);
          }
          _ => {}
        }
      (KeyCode::Home, _) | (KeyCode::Left, KeyModifiers::CONTROL) =>
        match self.focused_el {
          Focusable::Editor(i, _) => {
            self.focused_el = Focusable::Table(i);
          }
          _ => {}
        }
      (KeyCode::Enter, _) if
        match self.focused_el {
          Focusable::Table(_) | Focusable::Editor(_, EditorFocusable::LyricsButton) => true,
          _ => false,
        }
      => {
        match self.focused_el {
          Focusable::Table(i) => {
            self.focused_el = Focusable::Editor(i, EditorFocusable::TitleInput);
          }
          Focusable::Editor(i, editor_section) =>
            match editor_section {
              EditorFocusable::LyricsButton => {
                return Vec::from([UiCommand::ChangeScreen(ui_enums::ScreenKind::Lyrics)]);
              }
              _ => {}
            }
          _ => {}
        }
      }
      (KeyCode::Char('s' | 'ы'), KeyModifiers::CONTROL) => {
        // REPEATED
        let audio = match self.focused_el {
          Focusable::Table(i) | Focusable::Editor(i, _) =>
            Some((i, &mut state.searched_mp3_files[i].tags)),
          _ => None,
        };
        let i = audio.as_ref().map(|s| s.0);
        let mut tags = audio.map(|s| s.1);
        // REPEATED
        if let Some(tags) = tags {
          if !tags.edited() {
            return Vec::new();
          }
          return Vec::from([
            UiCommand::OpenModal(modal::enums::Modal::Save {
              index: i.unwrap(),
              song_title: tags.title.0.to_string(),
            }),
          ]);
        }
      }
      (KeyCode::Char('r' | 'к'), KeyModifiers::CONTROL) =>
        match self.focused_el {
          Focusable::Search => {
            state.search = String::default();
            state.search_mp3_files(state.search.clone());
          }
          Focusable::Editor(_, ed_f) => {
            // REPEATED
            let audio = match &self.focused_el {
              Focusable::Table(i) | Focusable::Editor(i, _) =>
                Some((i, &mut state.searched_mp3_files[*i].tags)),
              _ => None,
            };
            let i = audio.as_ref().map(|s| *s.0);
            let mut tags = audio.map(|s| s.1);
            // REPEATED
            tags.map(|tags| {
              match ed_f {
                EditorFocusable::TitleInput => tags.title.0.reset(),
                EditorFocusable::ArtistInput => tags.artist.0.reset(),
                EditorFocusable::YearInput => tags.year.0.reset(),
                EditorFocusable::GenreInput => tags.genre.0.reset(),
                EditorFocusable::LyricsButton => {}
              }
            });
          }
          _ => {}
        }
      _ =>
        match &self.focused_el {
          Focusable::Search => {
            if self.search_input.input_for_humans(key_event, false) {
              state.search_mp3_files(state.search.clone());
            }
            state.search = self.search_input.lines()[0].clone();
          }
          Focusable::Editor(i, editor_section) => {
            let tags = &mut state.searched_mp3_files[*i].tags;
            match editor_section {
              EditorFocusable::TitleInput => {
                self.title_input.input_for_humans(key_event, false);
                tags.title.0.edit(self.title_input.lines()[0].clone());
              }
              EditorFocusable::ArtistInput => {
                self.artist_input.input_for_humans(key_event, false);
                tags.artist.0.edit(self.artist_input.lines()[0].clone());
              }
              EditorFocusable::YearInput => {
                self.year_input.input_for_humans(key_event, false);
                tags.year.0.edit(self.year_input.lines()[0].clone());
              }
              EditorFocusable::GenreInput => {
                self.genre_input.input_for_humans(key_event, false);
                tags.genre.0.edit(self.genre_input.lines()[0].clone());
              }
              EditorFocusable::LyricsButton => {}
            }
          }
          _ => {}
        }
      _ => {}
    }
    Vec::new()
  }
}

fn update_inputs(home_screen: &mut HomeScreen, state: &mut State) {
  // REPEATED
  let audio = match &home_screen.focused_el {
    Focusable::Table(i) | Focusable::Editor(i, _) =>
      Some((i, &mut state.searched_mp3_files[*i].tags)),
    _ => None,
  };
  let i = audio.as_ref().map(|s| *s.0);
  let mut tags = audio.map(|s| s.1);
  // REPEATED
  home_screen.title_input.set_text(
    tags
      .as_ref()
      .map(|t| t.title.0.to_string())
      .unwrap_or_default()
  );
  home_screen.artist_input.set_text(
    tags
      .as_ref()
      .map(|t| t.artist.0.to_string())
      .unwrap_or_default()
  );
  home_screen.genre_input.set_text(
    tags
      .as_ref()
      .map(|t| t.genre.0.to_string())
      .unwrap_or_default()
  );
  home_screen.year_input.set_text(
    tags
      .as_ref()
      .map(|t| t.year.0.to_string())
      .unwrap_or_default()
  );
}
