use std::{
  fmt::{ self, Display },
  fs,
  io::Stdout,
  path::{ Path, PathBuf },
  sync::mpsc::{ channel, Receiver, Sender },
  time::{ Duration, SystemTime },
};
use chrono::{ DateTime, Local, Utc };
use crossterm::event::{ read, KeyEvent };
use ratatui::crossterm::event::{ self, poll, Event, KeyEventKind };
use humantime::format_duration;
use id3::{ frame::Lyrics, Tag, TagLike };
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use ratatui::{ prelude::{ Backend, CrosstermBackend }, Terminal };
use crate::ui::{
  home::{ self, screen::{ EditorFocusable, Focusable as HomeFocusable, HomeScreen } },
  lyrics::{ self, screen::Focusable as LyricsFocusable },
  modals::{ modal::enums::Modal, save_result::SaveTagsResultModal },
  text_area::TextAreaTrait,
  ui::Ui,
  ui_enums::{ self, Screen },
};
use super::{ state::{ Source, State }, tag::SongTags };

pub enum Command {
  Quit,

  OpenModal(Modal),
  CloseLastModal,
  SetModalOption(usize),
  ExecuteModalOption(usize),

  SetScreen(ui_enums::Screen),

  FocusHomeElement(HomeFocusable),
  FocusLyricsElement(LyricsFocusable),

  ResetHomeScreenTag(HomeFocusable),
  ResetLyricsScreenTag(LyricsFocusable),

  HandleHomeScreenInput(KeyEvent, HomeFocusable),
  HandleLyricsScreenInput(KeyEvent, LyricsFocusable),

  SaveTags(usize),
  TagsSaved(Result<(), String>),
}

#[derive(Clone)]
pub struct Mp3File {
  pub name: String,
  pub path: String,
  pub source: Source,
  pub modified_date: String,
  pub tags: SongTags,
}

pub struct App {
  commands_channel: (Sender<Command>, Receiver<Command>),
  pub state: State,
  ui: Ui,
}

impl App {
  pub fn new() -> Self {
    Self {
      commands_channel: channel(),
      state: State::new(),
      ui: Ui::new(),
    }
  }
  pub fn poll(&mut self) {
    if let Ok(cmd) = self.commands_channel.1.try_recv() {
      self.handle_command(cmd, self.commands_channel.0.clone());
    }
    if poll(Duration::from_millis(100)).unwrap() {
      let event = read().unwrap();
      self.ui.handle_input(&self.state, event, self.commands_channel.0.clone());
    }
    self.ui.render(&self.state);
  }
  fn handle_command(&mut self, cmd: Command, sender: Sender<Command>) {
    match cmd {
      Command::Quit => {
        self.state.running = false;
        match &mut self.ui.state.screen {
          Screen::Home(_) => {}
          // TODO: ???
          Screen::Lyrics(screen) => {
            self.ui.state.screen = Screen::Home(
              HomeScreen::new(
                HomeFocusable::Editor(screen.index, EditorFocusable::LyricsButton),
                Some({
                  let tags = &mut self.state.get_file_mut(screen.index).tags;
                  tags.lyrics = screen.lyrics.clone();
                  tags
                })
              )
            );
          }
        }
      }
      Command::SetScreen(screen) => {
        self.ui.state.screen = screen;
      }
      Command::SetModalOption(i) => {
        if let Some(last) = self.ui.state.modals.last_mut() {
          if let Some(with_options) = last.options_mut() {
            with_options.select(i);
          }
        }
      }
      Command::OpenModal(modal) => {
        self.ui.state.modals.open(modal);
      }
      Command::CloseLastModal => {
        self.ui.state.modals.close_last();
      }
      Command::ExecuteModalOption(i) => {
        let cmd = &mut self.ui.state.modals.last_mut().unwrap().options_mut().unwrap();
        let cmd = cmd.list_mut()[i].cmd.take().unwrap();
        self.handle_command(cmd, sender);
      }
      Command::FocusHomeElement(el) => {
        match &mut self.ui.state.screen {
          ui_enums::Screen::Home(screen) => {
            screen.focused_el = el;
            match el {
              HomeFocusable::Search => {
                screen.title_input.clear();
                screen.artist_input.clear();
                screen.year_input.clear();
                screen.genre_input.clear();
              }
              HomeFocusable::Table(i) => {
                let tags = &self.state.get_file(i).tags;
                screen.title_input.set_text(tags.title.0.to_string());
                screen.artist_input.set_text(tags.artist.0.to_string());
                screen.year_input.set_text(tags.year.0.to_string());
                screen.genre_input.set_text(tags.genre.0.to_string());
              }
              HomeFocusable::Editor(..) => {}
            }
          }
          _ => {}
        }
      }
      Command::FocusLyricsElement(el) => {
        match &mut self.ui.state.screen {
          ui_enums::Screen::Lyrics(screen) => {
            screen.focused_el = el;
          }
          _ => {}
        }
      }
      Command::SaveTags(i) => {
        let audio = &mut self.state.get_file_mut(i).tags;
        let res = audio.save();
        sender.send(Command::TagsSaved(res));
      }
      Command::TagsSaved(res) => {
        self.handle_command(
          Command::OpenModal(Modal::SaveTagsResult(SaveTagsResultModal::new(res))),
          sender
        );
      }
      Command::ResetHomeScreenTag(el) => {
        match &mut self.ui.state.screen {
          ui_enums::Screen::Home(screen) => {
            match el {
              HomeFocusable::Search => {
                self.state.search = String::new();
                self.state.search_mp3_files(String::new());
              }
              HomeFocusable::Table(_) => {}
              HomeFocusable::Editor(i, f_ed_el) => {
                let mut tags = &mut self.state.get_file_mut(i).tags;
                match f_ed_el {
                  EditorFocusable::TitleInput => {
                    tags.title.0.reset();
                    screen.title_input.set_text(tags.title.0.to_string())
                  }
                  EditorFocusable::ArtistInput => {
                    tags.artist.0.reset();
                    screen.artist_input.set_text(tags.artist.0.to_string())
                  }
                  EditorFocusable::YearInput => {
                    tags.year.0.reset();
                    screen.year_input.set_text(tags.year.0.to_string())
                  }
                  EditorFocusable::GenreInput => {
                    tags.genre.0.reset();
                    screen.genre_input.set_text(tags.genre.0.to_string())
                  }
                  EditorFocusable::LyricsButton => {
                    tags.lyrics.lang.reset();
                    tags.lyrics.desc.reset();
                    tags.lyrics.text.reset();
                  }
                }
              }
            }
          }
          _ => {}
        }
      }
      Command::ResetLyricsScreenTag(el) => {
        match &self.ui.state.screen {
          Screen::Lyrics(screen) => {
            let mut tags = &mut self.state.get_file_mut(screen.index).tags;
            match screen.focused_el {
              LyricsFocusable::Lang => tags.lyrics.lang.reset(),
              LyricsFocusable::Desc => tags.lyrics.desc.reset(),
              LyricsFocusable::Text => tags.lyrics.text.reset(),
            }
          }
          _ => {}
        }
      }
      Command::HandleHomeScreenInput(key_event, el) => {
        match &mut self.ui.state.screen {
          ui_enums::Screen::Home(screen) => {
            match el {
              HomeFocusable::Search => {
                if screen.search_input.input_for_humans(key_event, false) {
                  self.state.search_mp3_files(self.state.search.clone());
                }
                self.state.search = screen.search_input.lines()[0].clone();
              }
              HomeFocusable::Editor(i, editor_section) => {
                let tags = &mut self.state.get_file_mut(i).tags;
                match editor_section {
                  EditorFocusable::TitleInput => {
                    if screen.title_input.input_for_humans(key_event, false) {
                      tags.title.0.edit(screen.title_input.lines()[0].clone());
                    }
                  }
                  EditorFocusable::ArtistInput => {
                    if screen.artist_input.input_for_humans(key_event, false) {
                      tags.artist.0.edit(screen.artist_input.lines()[0].clone());
                    }
                  }
                  EditorFocusable::YearInput => {
                    if screen.year_input.input_for_humans(key_event, false) {
                      tags.year.0.edit(screen.year_input.lines()[0].clone());
                    }
                  }
                  EditorFocusable::GenreInput => {
                    if screen.genre_input.input_for_humans(key_event, false) {
                      tags.genre.0.edit(screen.genre_input.lines()[0].clone());
                    }
                  }
                  EditorFocusable::LyricsButton => {}
                }
              }
              _ => {}
            }
          }
          _ => {}
        }
      }
      Command::HandleLyricsScreenInput(key_event, el) => {
        match &mut self.ui.state.screen {
          ui_enums::Screen::Lyrics(screen) => {
            let tags = &mut self.state.get_file_mut(screen.index).tags;
            match el {
              LyricsFocusable::Lang => {
                if screen.lang_input.input_for_humans(key_event, false) {
                  tags.lyrics.lang.edit(screen.lang_input.lines()[0].clone());
                }
              }
              LyricsFocusable::Desc => {
                if screen.desc_input.input_for_humans(key_event, false) {
                  tags.lyrics.desc.edit(screen.desc_input.lines()[0].clone());
                }
              }
              LyricsFocusable::Text => {
                if screen.text_textarea.input_for_humans(key_event, true) {
                  tags.lyrics.text.edit(screen.text_textarea.lines()[0].clone());
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
