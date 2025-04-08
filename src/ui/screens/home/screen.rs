use std::sync::mpsc::Sender;

use crate::{
  app::{ app::{ App, Command }, state::State, tag::SongTags },
  info::{ PROJECT_DESC, PROJECT_NAME },
  ui::{
    block::BlockTrait,
    lyrics::screen::LyricsScreen,
    modals::{ help::HelpModal, modal::{ self, enums::Modal }, save_tags::ConfirmSaveTagsModal },
    shortcut::Shortcut,
    text_area::TextAreaTrait,
    ui::{ ui_enums, Ui },
    ui_enums::Screen,
    widget::{ FocusableWidget, WidgetWithEditableContent },
    InputHandler,
    StateDependentWidget,
    StringTrait,
    StyleFlags,
    UiState,
  },
};
use crossterm::event::{ Event, KeyCode, KeyEvent, KeyModifiers };
use id3::TagLike;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Flex, Layout, Margin, Rect },
  style::{ Color, Style, Styled, Stylize },
  text::{ Line, Span, Text },
  widgets::{
    block::Title,
    Block,
    BorderType,
    Cell,
    List,
    Paragraph,
    Row,
    StatefulWidget,
    Table,
    TableState,
    Widget,
  },
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
  pub search_input: TextArea<'static>,
  pub title_input: TextArea<'static>,
  pub artist_input: TextArea<'static>,
  pub year_input: TextArea<'static>,
  pub genre_input: TextArea<'static>,
}

impl HomeScreen {
  pub fn new(selection: Focusable, tags: Option<&SongTags>) -> Self {
    Self {
      focused_el: selection,
      search_input: {
        let mut input = TextArea::new(Vec::new());
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Search"));
        input.set_cursor_line_style(Style::new());
        input
      },
      title_input: {
        let mut input = TextArea::new(
          Vec::from([tags.map(|t| t.title.0.to_string()).unwrap_or_default()])
        );
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Title"));
        input.set_cursor_line_style(Style::new());
        input
      },
      artist_input: {
        let mut input = TextArea::new(
          Vec::from([tags.map(|t| t.artist.0.to_string()).unwrap_or_default()])
        );
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Artist"));
        input.set_cursor_line_style(Style::new());
        input
      },
      genre_input: {
        let mut input = TextArea::new(
          Vec::from([tags.map(|t| t.genre.0.to_string()).unwrap_or_default()])
        );
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Genre"));
        input.set_cursor_line_style(Style::new());
        input
      },
      year_input: {
        let mut input = TextArea::new(
          Vec::from([tags.map(|t| t.year.0.to_string()).unwrap_or_default()])
        );
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Year"));
        input.set_cursor_line_style(Style::new());
        input
      },
    }
  }
}

impl InputHandler for HomeScreen {
  fn handle_input(
    &self,
    state: &State,
    ui_state: &UiState,
    event: Event,
    sender: Sender<Command>
  ) -> bool {
    let shown_indexes = state.shown_indexes.len();
    match event {
      Event::Key(event) => {
        match (event.code, event.modifiers, self.focused_el) {
          (KeyCode::Esc, _, _) => {
            sender.send(Command::Quit);
            true
          }
          (KeyCode::Up, KeyModifiers::CONTROL, f_el) | (KeyCode::PageUp, _, f_el) =>
            match f_el {
              Focusable::Table(_) => {
                sender.send(Command::FocusHomeElement(Focusable::Search));
                true
              }
              Focusable::Editor(i, editor_selection) => {
                sender.send(
                  Command::FocusHomeElement(match editor_selection {
                    EditorFocusable::TitleInput =>
                      Focusable::Editor(i, EditorFocusable::LyricsButton),
                    EditorFocusable::ArtistInput =>
                      Focusable::Editor(i, EditorFocusable::TitleInput),
                    EditorFocusable::YearInput =>
                      Focusable::Editor(i, EditorFocusable::ArtistInput),
                    EditorFocusable::GenreInput => Focusable::Editor(i, EditorFocusable::YearInput),
                    EditorFocusable::LyricsButton =>
                      Focusable::Editor(i, EditorFocusable::GenreInput),
                  })
                );
                true
              }
              Focusable::Search => false,
            }
          (KeyCode::Up, KeyModifiers::NONE, Focusable::Table(i)) => {
            sender.send(
              Command::FocusHomeElement(
                Focusable::Table(if i > 0 { i - 1 } else { shown_indexes - 1 })
              )
            );
            true
          }
          (KeyCode::End, _, f_el) | (KeyCode::Right, KeyModifiers::CONTROL, f_el) =>
            match f_el {
              Focusable::Table(i) => {
                sender.send(
                  Command::FocusHomeElement(Focusable::Editor(i, EditorFocusable::TitleInput))
                );
                true
              }
              Focusable::Editor(i, ed_f_el) =>
                match ed_f_el {
                  EditorFocusable::LyricsButton => {
                    sender.send(
                      Command::SetScreen(
                        Screen::Lyrics(LyricsScreen::new(i, state.get_file(i).tags.lyrics.clone()))
                      )
                    );
                    true
                  }
                  _ => false,
                }
              _ => false,
            }
          (KeyCode::Down, KeyModifiers::CONTROL, f_el) | (KeyCode::PageDown, _, f_el) =>
            match f_el {
              Focusable::Search if shown_indexes > 0 => {
                sender.send(Command::FocusHomeElement(Focusable::Table(0)));
                true
              }
              Focusable::Table(..) => false,
              Focusable::Editor(i, editor_selection) => {
                sender.send(
                  Command::FocusHomeElement(match editor_selection {
                    EditorFocusable::TitleInput =>
                      Focusable::Editor(i, EditorFocusable::ArtistInput),
                    EditorFocusable::ArtistInput =>
                      Focusable::Editor(i, EditorFocusable::YearInput),
                    EditorFocusable::YearInput => Focusable::Editor(i, EditorFocusable::GenreInput),
                    EditorFocusable::GenreInput =>
                      Focusable::Editor(i, EditorFocusable::LyricsButton),
                    EditorFocusable::LyricsButton =>
                      Focusable::Editor(i, EditorFocusable::TitleInput),
                  })
                );
                true
              }
              _ => false,
            }
          (KeyCode::Down, KeyModifiers::NONE, Focusable::Table(i)) => {
            sender.send(
              Command::FocusHomeElement(
                Focusable::Table(if i == shown_indexes - 1 { 0 } else { i + 1 })
              )
            );
            true
          }
          (KeyCode::Home, _, f_el) | (KeyCode::Left, KeyModifiers::CONTROL, f_el) =>
            match f_el {
              Focusable::Editor(i, _) => {
                sender.send(Command::FocusHomeElement(Focusable::Table(i)));
                true
              }
              _ => false,
            }
          (
            KeyCode::Enter,
            _,
            f_el @ (Focusable::Table(..) | Focusable::Editor(_, EditorFocusable::LyricsButton)),
          ) => {
            match f_el {
              Focusable::Table(i) => {
                sender.send(
                  Command::FocusHomeElement(Focusable::Editor(i, EditorFocusable::TitleInput))
                );
                true
              }
              Focusable::Editor(i, f_ed_el) =>
                match f_ed_el {
                  EditorFocusable::LyricsButton => {
                    sender.send(
                      Command::SetScreen(
                        Screen::Lyrics(LyricsScreen::new(i, state.get_file(i).tags.lyrics.clone()))
                      )
                    );
                    true
                  }
                  _ => false,
                }
              _ => false,
            }
          }
          (
            KeyCode::Char('s' | 'ы'),
            KeyModifiers::CONTROL,
            Focusable::Table(i) | Focusable::Editor(i, _),
          ) => {
            let tags = &state.get_file(i).tags;
            if tags.edited() {
              sender.send(
                Command::OpenModal(
                  Modal::ConfirmSaveTags(ConfirmSaveTagsModal::new(i, tags.title.0.to_string()))
                )
              );
            }
            true
          }
          (KeyCode::Char('h' | 'р'), KeyModifiers::CONTROL, _) => {
            sender.send(Command::OpenModal(Modal::Help(HelpModal)));
            true
          }
          (KeyCode::Char('r' | 'к'), KeyModifiers::CONTROL, f_el) => {
            sender.send(Command::ResetHomeScreenTag(f_el));
            true
          }
          _ => {
            sender.send(Command::HandleHomeScreenInput(event.clone(), self.focused_el));
            true
          }
          _ => false,
        }
      }
      _ => false,
    }
  }
}

impl StateDependentWidget for HomeScreen {
  fn render_from_state(&self, area: Rect, buf: &mut Buffer, state: &State, ui_state: &UiState)
    where Self: Sized
  {
    let github_shortcut = Shortcut::new("Ctrl+G", "Github", Color::Gray);
    let help_shortcut = Shortcut::new("Ctrl+H", "Help", Color::Gray);
    let save_shortcut = Shortcut::new("Ctrl+S", "Save", Color::Yellow);

    let [header_area, main_area, footer_area] = Layout::vertical([
      Constraint::Length(1),
      Constraint::Fill(1),
      Constraint::Length(1),
    ]).areas(area);
    let footer_area = footer_area.inner(Margin::new(1, 0));
    let [sidebar_area, editor_area] = Layout::horizontal([
      Constraint::Fill(1),
      Constraint::Fill(1),
    ]).areas(main_area);
    let [search_area, table_area] = Layout::vertical([
      Constraint::Length(3),
      Constraint::Fill(1),
    ]).areas(sidebar_area);

    {
      let mut search_input = self.search_input.clone();
      let flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: self.focused_el == Focusable::Search,
      };
      search_input.set_style(
        Style::from(StyleFlags {
          enabled: true,
          valid: true,
          highlighted: false,
        })
      );
      search_input.set_block(
        search_input.block().cloned().unwrap_or_default().border_style(Style::from(flags))
      );
      search_input.toggle_cursor(flags.highlighted);
      search_input.render(search_area, buf);
    }

    let sel_song_i = match &self.focused_el {
      Focusable::Table(i) | Focusable::Editor(i, ..) => Some(i),
      _ => None,
    };

    {
      let mut files_table = {
        Table::new(
          state.shown_indexes
            .iter()
            .map(|i| {
              let f = state.get_file(*i);
              let edited = f.tags.edited();
              Row::new(
                vec![
                  Cell::from(
                    Line::from(
                      Vec::from([
                        if edited { Span::from("▌").yellow() } else { Span::from(" ") },
                        Span::from(f.name.clone()).style(
                          if edited {
                            Style::new().yellow()
                          } else {
                            Style::new()
                          }
                        ),
                      ])
                    )
                  ),
                  Cell::new(f.source.to_string().green().italic()),
                  Cell::new(f.modified_date.clone().dark_gray())
                ]
              )
            })
            .collect::<Vec<_>>(),
          [Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)]
        )
          .row_highlight_style(Style::new().on_dark_gray().bold())
          .column_spacing(1)
          .block(
            Block::bordered()
              .border_type(BorderType::Rounded)
              .border_style(
                Style::from(StyleFlags {
                  enabled: true,
                  valid: true,
                  highlighted: match self.focused_el {
                    Focusable::Table(..) => true,
                    _ => false,
                  },
                })
              )
              .title_bottom(
                sel_song_i.map_or(Line::default(), |i|
                  Line::from(
                    Vec::from([
                      Span::from(" "),
                      Span::from((i + 1).to_string()).gray(),
                      Span::from("/").dark_gray(),
                      Span::from(state.shown_indexes.len().to_string().dark_gray()),
                      Span::from(" "),
                    ])
                  ).centered()
                )
              )
          )
      };
      let mut table_state = &mut TableState::new().with_selected(sel_song_i.cloned());
      <Table as StatefulWidget>::render(files_table, table_area, buf, table_state);
    }

    {
      let [file_area, path_area, mod_area] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
      ])
        .horizontal_margin(1)
        .spacing(1)
        .areas(table_area);

      let table_title_file = Line::from(" MP3 file ").centered().render(file_area, buf);
      let table_title_path = Line::from(" Path ").centered().render(path_area, buf);
      let table_title_mod = Line::from(" Modified ").centered().render(mod_area, buf);
    }

    let tags = match &self.focused_el {
      Focusable::Table(i) | Focusable::Editor(i, _) => Some(&state.get_file(*i).tags),
      _ => None,
    };

    let editor_focused = match self.focused_el {
      Focusable::Editor(..) => true,
      _ => false,
    };

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

    {
      let mut title_input = self.title_input.clone();
      let border_flags = StyleFlags {
        enabled: editor_focused,
        valid: true,
        highlighted: match self.focused_el {
          Focusable::Editor(_, ed_f) =>
            match ed_f {
              EditorFocusable::TitleInput => true,
              _ => false,
            }
          _ => false,
        },
      };
      title_input.set_style(
        Style::from(StyleFlags {
          enabled: editor_focused,
          valid: true,
          highlighted: tags.map(|t| t.title.0.edited()).unwrap_or_default(),
        })
      );
      title_input.set_block(
        title_input.block().cloned().unwrap_or_default().border_style(Style::from(border_flags))
      );
      title_input.toggle_cursor(border_flags.highlighted);
      title_input.render(title_input_area, buf);
    }

    {
      let mut artist_input = self.artist_input.clone();
      let flags = StyleFlags {
        enabled: editor_focused,
        valid: true,
        highlighted: match self.focused_el {
          Focusable::Editor(_, ed_f) =>
            match ed_f {
              EditorFocusable::ArtistInput => true,
              _ => false,
            }
          _ => false,
        },
      };
      artist_input.set_style(
        Style::from(StyleFlags {
          enabled: editor_focused,
          valid: true,
          highlighted: tags.map(|t| t.artist.0.edited()).unwrap_or_default(),
        })
      );
      artist_input.set_block(
        artist_input.block().cloned().unwrap_or_default().border_style(Style::from(flags))
      );
      artist_input.toggle_cursor(flags.highlighted);
      artist_input.render(artist_input_area, buf);
    }

    {
      let mut year_input = self.year_input.clone();
      let flags = StyleFlags {
        enabled: editor_focused,
        valid: true,
        highlighted: match self.focused_el {
          Focusable::Editor(_, ed_f) =>
            match ed_f {
              EditorFocusable::YearInput => true,
              _ => false,
            }
          _ => false,
        },
      };
      year_input.set_style(
        Style::from(StyleFlags {
          enabled: editor_focused,
          valid: true,
          highlighted: tags.map(|t| t.year.0.edited()).unwrap_or_default(),
        })
      );
      year_input.set_block(
        year_input.block().cloned().unwrap_or_default().border_style(Style::from(flags))
      );
      year_input.toggle_cursor(flags.highlighted);
      year_input.render(year_input_area, buf);
    }

    {
      let mut genre_input = self.genre_input.clone();
      let flags = StyleFlags {
        enabled: editor_focused,
        valid: true,
        highlighted: match self.focused_el {
          Focusable::Editor(_, ed_f) =>
            match ed_f {
              EditorFocusable::GenreInput => true,
              _ => false,
            }
          _ => false,
        },
      };
      genre_input.set_style(
        Style::from(StyleFlags {
          enabled: editor_focused,
          valid: true,
          highlighted: tags.map(|t| t.genre.0.edited()).unwrap_or_default(),
        })
      );
      genre_input.set_block(
        genre_input.block().cloned().unwrap_or_default().border_style(Style::from(flags))
      );
      genre_input.toggle_cursor(flags.highlighted);
      genre_input.render(genre_input_area, buf);
    }

    let mut lyrics_button = Paragraph::new(
      vec![
        Line::default(),
        Line::from("Edit").style(
          Style::from(StyleFlags {
            enabled: editor_focused,
            valid: true,
            highlighted: tags
              .map(|t| (t.lyrics.lang.edited() || t.lyrics.desc.edited() || t.lyrics.text.edited()))
              .unwrap_or_default(),
          })
        ),
        Line::default()
      ]
    )
      .block(
        Block::bordered()
          .border_type(BorderType::Rounded)
          .title_top("Lyrics")
          .border_style(
            Style::from(StyleFlags {
              enabled: editor_focused,
              highlighted: match self.focused_el {
                Focusable::Editor(_, ed_f) =>
                  match ed_f {
                    EditorFocusable::LyricsButton => true,
                    _ => false,
                  }
                _ => false,
              },
              valid: true,
            })
          )
      )
      .centered()
      .render(lyrics_button_area, buf);

    let header_line = Line::from(
      Vec::from([
        Span::from(": ").dark_gray(),
        Span::from(PROJECT_NAME),
        Span::from(" :").dark_gray(),
      ])
    ).render(header_area, buf);

    let footer_line = Line::from(
      Vec::from([
        if tags.is_some_and(|t| t.edited()) { save_shortcut.to_spans() } else { Vec::new() },
        if tags.is_some_and(|t| t.edited()) {
          Vec::from([Span::from(" :: ").dark_gray()])
        } else {
          Vec::new()
        },
        help_shortcut.to_spans(),
        Vec::from([Span::from(" :: ").dark_gray()]),
        github_shortcut.to_spans(),
      ]).concat()
    )
      .right_aligned()
      .render(footer_area, buf);
  }
}
