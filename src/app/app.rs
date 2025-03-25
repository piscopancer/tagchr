use std::{ fs, path::{ Path, PathBuf }, time::SystemTime };
use chrono::{ DateTime, Local, Utc };
use humantime::format_duration;
use id3::{ Tag, TagLike };
use pretty_date::pretty_date_formatter::PrettyDateFormatter;

#[derive(Debug)]
pub struct Mp3File {
  pub name: String,
  pub path: String,
  pub modified_date: String,
}

#[derive(Clone, Default)]
pub enum TagState {
  #[default]
  Unchanged,
  Changed(String),
  Removed,
}

#[derive(Clone)]
pub struct EditableTag {
  pub original: Option<String>,
  pub state: TagState,
}

impl EditableTag {
  pub fn new(original: Option<String>) -> Self {
    Self { original, state: TagState::default() }
  }
  pub fn edit(&mut self, new: String) {
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

#[derive(Clone)]
pub struct SongTags {
  pub name: EditableTag,
  pub artist: EditableTag,
  pub year: EditableTag,
  pub genre: EditableTag,
}

impl SongTags {
  pub fn new(path: String) -> Self {
    let tag = Tag::read_from_path(path).unwrap();
    Self {
      name: EditableTag::new(tag.title().map(|n| n.into())),
      artist: EditableTag::new(tag.artist().map(|a| a.into())),
      year: EditableTag::new(tag.year().map(|y| y.to_string())),
      genre: EditableTag::new(tag.genre().map(|g| g.to_string())),
    }
  }
}

pub struct App {
  pub running: bool,
  pub path: String,
  pub found_mp3_files: Vec<Mp3File>,
  pub selected_song: Option<SongTags>,
}

impl App {
  pub fn new() -> Self {
    let mut app = App {
      path: "".into(),
      running: true,
      found_mp3_files: vec![],
      selected_song: None,
    };
    dirs::download_dir().map(|dir| app.find_mp3_files(dir));
    dirs::audio_dir().map(|dir| app.find_mp3_files(dir));
    app
  }
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
