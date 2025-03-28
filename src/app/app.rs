use std::{ fs, io::Stdout, path::{ Path, PathBuf }, time::{ Duration, SystemTime } };
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
}

#[derive(Clone, Default)]
pub enum TagState<T> {
  #[default]
  Unchanged,
  Changed(T),
  Removed,
}

#[derive(Clone)]
pub struct EditableTag<T> {
  pub original: Option<T>,
  pub state: TagState<T>,
}

pub trait EditableTagTrait<T> {
  fn edit(&mut self, new: T);
}

impl<T> EditableTag<T> {
  pub fn new(original: Option<T>) -> Self {
    Self { original, state: TagState::default() }
  }
}

impl EditableTagTrait<String> for EditableTag<String> {
  fn edit(&mut self, new: String) {
    self.state = if let Some(original) = &self.original {
      if *original == new {
        TagState::Unchanged
      } else if new.chars().count() == 0 {
        TagState::Removed
      } else {
        TagState::Changed(new)
      }
    } else {
      TagState::Changed(new)
    };
  }
}

impl EditableTagTrait<Lyrics> for EditableTag<Lyrics> {
  fn edit(&mut self, new: Lyrics) {
    self.state = if let Some(original) = &self.original {
      if
        original.description == new.description ||
        original.lang == new.lang ||
        original.text == new.text
      {
        TagState::Unchanged
      } else if
        new.description.chars().count() == 0 ||
        new.lang.chars().count() == 0 ||
        new.text.chars().count() == 0
      {
        TagState::Removed
      } else {
        TagState::Changed(new)
      }
    } else {
      TagState::Changed(new)
    };
  }
}

#[derive(Clone)]
pub struct SongTags {
  pub name: EditableTag<String>,
  pub artist: EditableTag<String>,
  pub year: EditableTag<String>,
  pub genre: EditableTag<String>,
  pub lyrics: EditableTag<Lyrics>,
}

impl SongTags {
  pub fn new(path: String) -> Self {
    let tag = Tag::read_from_path(path).unwrap();
    Self {
      name: EditableTag::new(tag.title().map(|n| n.into())),
      artist: EditableTag::new(tag.artist().map(|a| a.into())),
      year: EditableTag::new(tag.year().map(|y| y.to_string())),
      genre: EditableTag::new(tag.genre().map(|g| g.to_string())),
      lyrics: EditableTag::new({
        let l = tag.lyrics().next().cloned();
        l
      }),
    }
  }
}

pub struct State {
  pub running: bool,
  search: String,
  pub found_mp3_files: Vec<Mp3File>,
  pub selected_song: Option<SongTags>,
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
        selected_song: None,
      },
      ui: Ui::new(),
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
    match new_tags.name.state {
      TagState::Changed(name) => {
        tags.set_title(name);
      }
      TagState::Removed => {
        tags.remove_title();
      }
      _ => {}
    }
    match new_tags.artist.state {
      TagState::Changed(artist) => {
        tags.set_artist(artist);
      }
      TagState::Removed => {
        tags.remove_artist();
      }
      _ => {}
    }
    match new_tags.year.state {
      TagState::Changed(year) => {
        tags.set_year(year.parse().unwrap_or(0));
      }
      TagState::Removed => {
        tags.remove_year();
      }
      _ => {}
    }
    tags.write_to_path(path, id3::Version::Id3v24);
  }
}
