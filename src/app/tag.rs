use core::fmt;
use id3::{ frame::Lyrics, Tag, TagLike };

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
  pub fn reset(&mut self) {
    self.state = EditableState::Unchanged;
  }
  pub fn edited(&self) -> bool {
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
  song_path: String,
  pub title: EditableTag,
  pub artist: EditableTag,
  pub year: EditableTag,
  pub genre: EditableTag,
  pub lyrics: LyricsEditableTag,
}

impl SongTags {
  pub fn new(song_path: String) -> Self {
    let tag = match Tag::read_from_path(song_path.clone()) {
      Ok(x) => {x},
      Err(e) => {print!("{}", e); Tag::new()}
    };
    Self {
      song_path,
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
  pub fn edited(&self) -> bool {
    self.title.0.edited() ||
      self.artist.0.edited() ||
      self.year.0.edited() ||
      self.genre.0.edited() ||
      self.lyrics.lang.edited() ||
      self.lyrics.desc.edited() ||
      self.lyrics.text.edited()
  }
  pub fn save(&mut self) -> Result<(), String> {
    let mut tags = Tag::new();
    match &self.title.0.state {
      EditableState::Unchanged => {
        if let Some(t) = &self.title.0.original {
          tags.set_title(t);
        }
      }
      EditableState::Changed(name) => {
        tags.set_title(name);
      }
      EditableState::Removed => {
        tags.remove_title();
      }
    }
    match &self.artist.0.state {
      EditableState::Unchanged => {
        if let Some(a) = &self.artist.0.original {
          tags.set_artist(a);
        }
      }
      EditableState::Changed(artist) => {
        tags.set_artist(artist);
      }
      EditableState::Removed => {
        tags.remove_artist();
      }
    }
    match &self.year.0.state {
      EditableState::Unchanged => {
        if let Some(year) = &self.year.0.original {
          tags.set_year(year.parse().unwrap_or(0));
        }
      }
      EditableState::Changed(year) => {
        tags.set_year(year.parse().unwrap_or(0));
      }
      EditableState::Removed => {
        tags.remove_year();
      }
    }
    match &self.genre.0.state {
      EditableState::Unchanged => {
        tags.set_genre(self.genre.0.original.clone().unwrap_or_default());
      }
      EditableState::Changed(genre) => {
        tags.set_genre(genre);
      }
      EditableState::Removed => {
        tags.remove_genre();
      }
    }
    tags.add_lyrics(Lyrics {
      lang: {
        match &self.lyrics.lang.state {
          EditableState::Changed(lang) => lang.clone(),
          EditableState::Unchanged => self.lyrics.lang.original.clone().unwrap_or_default(),
          EditableState::Removed => String::default(),
        }
      },
      description: {
        match &self.lyrics.desc.state {
          EditableState::Changed(desc) => desc.clone(),
          EditableState::Unchanged => self.lyrics.desc.original.clone().unwrap_or_default(),
          EditableState::Removed => String::default(),
        }
      },
      text: {
        match &self.lyrics.text.state {
          EditableState::Changed(text) => text.clone(),
          EditableState::Unchanged => self.lyrics.text.original.clone().unwrap_or_default(),
          EditableState::Removed => String::default(),
        }
      },
    });
    let write_res = tags.write_to_path(self.song_path.clone(), id3::Version::Id3v24);
    match write_res {
      Ok(_) => {
        *self = Self::new(self.song_path.clone());
        Ok(())
      }
      Err(e) => { Err(e.description) }
    }
  }
}
