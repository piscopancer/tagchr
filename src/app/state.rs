use std::{ fmt, fs, path::PathBuf, string };
use chrono::{ DateTime, Local };
use id3::Error;
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use super::{ app::Mp3File, tag::SongTags };

#[derive(Clone, Copy)]
pub enum Source {
  Custom,
  Downloads,
  Music,
}

impl fmt::Display for Source {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Source::Custom => write!(f, "Custom"),
      Source::Downloads => write!(f, "~/Downloads"),
      Source::Music => write!(f, "~/Music"),
    }
  }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut file_result = fs::read_to_string(filename);
    
    match file_result {
      Ok(x) => {
        for line in x.lines() {
          result.push(line.to_string())
        }
      }
      Err(e) => {
        println!("Error while trying to open user directories file: {}", e);
      }
    }
      
    return result;
}

const DEFAULT_USER_DIRS_FILE_POSTFIX: &str = "/.tagchr/directories.txt";

pub struct State {
  pub running: bool,
  pub search: String,
  files: Vec<Mp3File>,
  pub directories: Vec<(std::path::PathBuf, Source)>,
  pub shown_indexes: Vec<usize>,
}

impl State {
  pub fn new() -> Self {
    let mut new = Self {
      running: true,
      search: "".into(),
      files: vec![],
      directories: vec![],
      shown_indexes: vec![],
    };

    let DEFAULT_USER_DIRS_FILE_PREFIX: &str = match dirs::home_dir() {
        Some(mut x) => { 
          &(x.to_str().unwrap_or("~").to_owned())
        },
        None => {"~"}
    };

    let DEFAULT_USER_DIRS_FILE = (DEFAULT_USER_DIRS_FILE_PREFIX).to_string() + DEFAULT_USER_DIRS_FILE_POSTFIX;

    // println!("{}", DEFAULT_USER_DIRS_FILE);

    let user_dirs: Vec<std::path::PathBuf> = read_lines(&DEFAULT_USER_DIRS_FILE).iter().map(
      |s| {
        let mut p = std::path::PathBuf::new();
        p.push(s);
        // p.canonicalize();
        return p;
      }
    ).collect();

    for dir in user_dirs {
      new.directories.push((dir, Source::Custom));
    }

    match dirs::download_dir() {
        Some(x) => {new.directories.push((x,Source::Downloads));}
        None => {}
    }
    match dirs::audio_dir() {
        Some(x) => {new.directories.push((x,Source::Music));}
        None => {}
    }
    
    let dirs = new.directories.clone();
    for (dir,src) in dirs {
      let canon_dir = dir.canonicalize();
      match canon_dir {
        Ok(value) => {  
          new.scan_mp3_files(value, src)
        }
        Err(e) => {
          println!("Cannot canonicalize path {:?}", dir.to_str());
        }
      }
    }
    
    new.search_mp3_files(new.search.clone());
    
    new
  }
  pub fn get_file(&self, i: usize) -> &Mp3File {
    &self.files[i]
  }
  pub fn get_file_mut(&mut self, i: usize) -> &mut Mp3File {
    &mut self.files[i]
  }
  fn scan_mp3_files(&mut self, path_input: PathBuf, source: Source) {
    if let Ok(entries) = fs::read_dir(path_input) {
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
