use std::{
  fmt::{ self, Display },
  fs,
  io::Stdout,
  path::{ Path, PathBuf },
  time::{ Duration, SystemTime },
};
use chrono::{ DateTime, Local, Utc };
use crossterm::event::{ self, poll, Event, KeyEventKind };
use humantime::format_duration;
use id3::{ frame::Lyrics, Tag, TagLike };
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use ratatui::{ prelude::{ Backend, CrosstermBackend }, Terminal };
use crate::ui::ui::Ui;

#[derive(Debug)]
pub struct Mp3File {
  pub name: String,
  pub path: String,
  pub modified_date: String,
  pub tags: SongTags,
}

#[derive(Clone, Default, Debug)]
pub enum EditableState {
  #[default]
  Unchanged,
  Changed(String),
  Removed,
}

impl EditableState {
  fn compare(original: Option<&String>, new: String) -> Self {
    if let Some(original) = original {
      if *original == new {
        EditableState::Unchanged
      } else if new.chars().count() == 0 {
        EditableState::Removed
      } else {
        EditableState::Changed(new)
      }
    } else {
      if new.chars().count() == 0 { EditableState::Unchanged } else { EditableState::Changed(new) }
    }
  }
}

#[derive(Clone, Default, Debug)]
pub struct Editable {
  pub original: Option<String>,
  pub state: EditableState,
}

impl Editable {
  pub fn new(original: Option<String>) -> Self {
    Self {
      original,
      state: EditableState::default(),
    }
  }
  pub fn edit(&mut self, new: String) {
    self.state = EditableState::compare(self.original.as_ref(), new);
  }
  pub fn changed(&self) -> bool {
    match &self.state {
      EditableState::Unchanged => false,
      _ => true,
    }
  }
}

impl fmt::Display for Editable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let current = match &self.state {
      EditableState::Unchanged => self.original.clone(),
      EditableState::Changed(v) => Some(v.clone()),
      EditableState::Removed => None,
    };
    write!(f, "{}", current.unwrap_or_default())
  }
}

#[derive(Clone, Default, Debug)]
pub struct EditableTag(pub Editable);

#[derive(Clone, Default, Debug)]
pub struct LyricsEditableTag {
  pub lang: Editable,
  pub desc: Editable,
  pub text: Editable,
}

impl LyricsEditableTag {
  pub fn new(lyrics: Option<Lyrics>) -> Self {
    Self {
      lang: Editable::new(lyrics.as_ref().map(|l| l.lang.clone())),
      desc: Editable::new(lyrics.as_ref().map(|l| l.description.clone())),
      text: Editable::new(lyrics.map(|l| l.text.clone())),
    }
  }
}

#[derive(Clone, Debug)]
pub struct SongTags {
  pub title: EditableTag,
  pub artist: EditableTag,
  pub year: EditableTag,
  pub genre: EditableTag,
  pub lyrics: LyricsEditableTag,
}

impl SongTags {
  pub fn new(path: String) -> Self {
    let tag = Tag::read_from_path(path).unwrap();
    Self {
      title: EditableTag(Editable::new(tag.title().map(|n| n.into()))),
      artist: EditableTag(Editable::new(tag.artist().map(|a| a.into()))),
      year: EditableTag(Editable::new(tag.year().map(|y| y.to_string()))),
      genre: EditableTag(Editable::new(tag.genre().map(|g| g.to_string()))),
      lyrics: LyricsEditableTag::new({
        let l = tag.lyrics().next().cloned();
        l
      }),
    }
  }
}

pub struct State {
  pub running: bool,
  pub search: String,
  pub found_mp3_files: Vec<Mp3File>,
  // pub selected_song_index: Option<usize>,
}

impl State {
  fn find_mp3_files(&mut self, path: PathBuf) {
    if let Ok(entries) = fs::read_dir(path) {
      let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
      entries.sort_by(|a, b|
        b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap())
      );
      for entry in entries {
        let path = entry.path();
        if path.is_dir() {
          self.find_mp3_files(path.into());
        } else if path.extension().map_or(false, |ext| ext == "mp3") {
          self.found_mp3_files.push(Mp3File {
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
        found_mp3_files: vec![],
        // selected_song_index: None,
      },
      ui: Ui::new(None),
    };
    dirs::download_dir().map(|dir| app.state.find_mp3_files(dir));
    dirs::audio_dir().map(|dir| app.state.find_mp3_files(dir));
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
  fn save_tags(path: impl AsRef<Path>, new_tags: SongTags) {
    let mut tags = Tag::new();
    match new_tags.title.0.state {
      EditableState::Changed(name) => {
        tags.set_title(name);
      }
      EditableState::Removed => {
        tags.remove_title();
      }
      _ => {}
    }
    match new_tags.artist.0.state {
      EditableState::Changed(artist) => {
        tags.set_artist(artist);
      }
      EditableState::Removed => {
        tags.remove_artist();
      }
      _ => {}
    }
    match new_tags.year.0.state {
      EditableState::Changed(year) => {
        tags.set_year(year.parse().unwrap_or(0));
      }
      EditableState::Removed => {
        tags.remove_year();
      }
      _ => {}
    }
    tags.write_to_path(path, id3::Version::Id3v24);
  }
}
