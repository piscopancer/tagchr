use std::{ fmt, fs, path::PathBuf };
use chrono::{ DateTime, Local };
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use super::{ app::Mp3File, tag::SongTags };

#[derive(Clone, Copy)]
pub enum Source {
  Downloads,
  Music,
}

impl fmt::Display for Source {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Source::Downloads => write!(f, "~/Downloads"),
      Source::Music => write!(f, "~/Music"),
    }
  }
}

pub struct State {
  pub running: bool,
  pub search: String,
  files: Vec<Mp3File>,
  pub shown_indexes: Vec<usize>,
}

impl State {
  pub fn new() -> Self {
    let mut new = Self {
      running: true,
      search: "".into(),
      files: vec![],
      shown_indexes: vec![],
    };
    dirs::download_dir().map(|dir| new.scan_mp3_files(dir, Source::Downloads));
    dirs::audio_dir().map(|dir| new.scan_mp3_files(dir, Source::Music));
    new.search_mp3_files(new.search.clone());
    new
  }
  pub fn get_file(&self, i: usize) -> &Mp3File {
    &self.files[i]
  }
  pub fn get_file_mut(&mut self, i: usize) -> &mut Mp3File {
    &mut self.files[i]
  }
  fn scan_mp3_files(&mut self, path: PathBuf, source: Source) {
    if let Ok(entries) = fs::read_dir(path) {
      let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
      entries.sort_by(|a, b|
        b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap())
      );
      for entry in entries {
        let path = entry.path();
        if path.is_dir() {
          self.scan_mp3_files(path.into(), source);
        } else if path.extension().map_or(false, |ext| ext == "mp3") {
          self.files.push(Mp3File {
            tags: SongTags::new(path.to_str().unwrap().into()),
            name: entry.file_name().to_str().unwrap().into(),
            path: path.to_str().unwrap().to_string().replace("\\", "/"),
            source,
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
      self.shown_indexes = (0..self.files.len()).collect();
      return;
    }
    self.shown_indexes = self.files
      .iter()
      .enumerate()
      .filter(
        |(i, f)|
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
      .map(|(i, f)| i)
      .collect();
  }
}
