use std::{
  fmt::{ self, Display },
  fs,
  io::Stdout,
  path::{ Path, PathBuf },
  time::{ Duration, SystemTime },
};
use chrono::{ DateTime, Local, Utc };
use ratatui::crossterm::event::{ self, poll, Event, KeyEventKind };
use humantime::format_duration;
use id3::{ frame::Lyrics, Tag, TagLike };
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use ratatui::{ prelude::{ Backend, CrosstermBackend }, Terminal };
use crate::ui::ui::Ui;
use super::tag::SongTags;

#[derive(Debug, Clone)]
pub struct Mp3File {
  pub name: String,
  pub path: String,
  pub modified_date: String,
  pub tags: SongTags,
}

pub struct State {
  pub running: bool,
  pub search: String,
  pub scanned_mp3_files: Vec<Mp3File>,
  pub searched_mp3_files: Vec<Mp3File>,
}

impl State {
  fn scan_mp3_files(&mut self, path: PathBuf) {
    if let Ok(entries) = fs::read_dir(path) {
      let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
      entries.sort_by(|a, b|
        b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap())
      );
      for entry in entries {
        let path = entry.path();
        if path.is_dir() {
          self.scan_mp3_files(path.into());
        } else if path.extension().map_or(false, |ext| ext == "mp3") {
          self.scanned_mp3_files.push(Mp3File {
            tags: SongTags::new(path.to_str().unwrap().into()),
            name: entry.file_name().to_str().unwrap().into(),
            path: path.to_str().unwrap().to_string().replace("\\", "/"),
            modified_date: {
              let modified_date = entry.metadata().unwrap().modified().unwrap();
              let modified_date: DateTime<Local> = DateTime::from(modified_date);
              modified_date.naive_local().format_pretty()
            },
          });
        }
      }
    }
  }
  pub fn search_mp3_files(&mut self, search: String) {
    let search = search.to_lowercase();
    if search.trim().is_empty() {
      self.searched_mp3_files = self.scanned_mp3_files.clone();
      return;
    }
    self.searched_mp3_files = self.scanned_mp3_files
      .iter()
      .cloned()
      .filter(
        |f|
          f.name.to_lowercase().contains(&search) ||
          f.tags.title.0.original
            .as_ref()
            .map(|t| t.to_lowercase().contains(&search))
            .unwrap_or_default() ||
          f.tags.artist.0.original
            .as_ref()
            .map(|a| a.to_lowercase().contains(&search))
            .unwrap_or_default() ||
          f.tags.genre.0.original
            .as_ref()
            .map(|g| g.to_lowercase().contains(&search))
            .unwrap_or_default() ||
          f.tags.lyrics.text.original
            .as_ref()
            .map(|t| t.to_lowercase().contains(&search))
            .unwrap_or_default()
      )
      .collect();
  }
}

pub struct App {
  pub state: State,
  ui: Ui,
}

impl App {
  pub fn new() -> Self {
    let mut app = App {
      state: State {
        running: true,
        search: "".into(),
        scanned_mp3_files: vec![],
        searched_mp3_files: vec![],
      },
      ui: Ui::new(),
    };
    dirs::download_dir().map(|dir| app.state.scan_mp3_files(dir));
    dirs::audio_dir().map(|dir| app.state.scan_mp3_files(dir));
    app.state.search_mp3_files(app.state.search.clone());
    app
  }
  pub fn poll(&mut self) {
    if poll(Duration::from_millis(100)).unwrap() {
      self.ui.draw(&mut self.state);
      match event::read().unwrap() {
        Event::Key(key_event) => if key_event.kind == KeyEventKind::Press {
          self.ui.handle_key_event(key_event, &mut self.state);
        }
        _ => {}
      }
    }
  }
}
