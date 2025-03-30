use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use id3::frame::Lyrics;
use ratatui::{
  layout::{ Constraint, Layout },
  style::Stylize,
  text::{ Line, Span },
  widgets::Paragraph,
  Frame,
};
use tui_textarea::TextArea;
use crate::{
  app::app::{ App, Editable, LyricsEditableTag, SongTags, State },
  ui::{
    home::{ self, screen::HomeScreen },
    ui::{
      basic_text_area,
      ui_enums,
      Screen,
      SelectableWidget,
      TextAreaTrait,
      Ui,
      UiCommand,
      WidgetWithEditableContent,
    },
  },
};

#[derive(Clone, Copy, PartialEq)]
enum SelectableItem {
  Lang,
  Desc,
  Text,
}

pub struct LyricsScreen {
  // file name
  // song total time or idk
  pub index: usize,
  pub lyrics: LyricsEditableTag,
  selection: SelectableItem,
  header_par: Paragraph<'static>,
  lang_input: TextArea<'static>,
  desc_input: TextArea<'static>,
  text_input: TextArea<'static>,
  footer_par: Paragraph<'static>,
}

impl LyricsScreen {
  pub fn new(index: usize, lyrics: LyricsEditableTag) -> Self {
    let selection = SelectableItem::Lang;
    Self {
      selection,
      header_par: Paragraph::new(
        vec![Line::from("Lyrics Editing"), Line::from("").gray()]
      ).centered(),
      // TODO: when inputs first appear they must decide whether they need to be highlighted based on the edited tag state. should create a struct `Editable`
      lang_input: basic_text_area("Language".into(), Some(lyrics.lang.to_string())).focused(
        selection == SelectableItem::Lang
      ),
      desc_input: basic_text_area("Description".into(), Some(lyrics.desc.to_string())).focused(
        selection == SelectableItem::Desc
      ),
      text_input: basic_text_area("Text".into(), Some(lyrics.text.to_string())).focused(
        selection == SelectableItem::Text
      ),
      footer_par: Paragraph::new(
        Line::from(
          vec![Span::from("<Ctrl-S>").yellow(), Span::from(":").dark_gray(), Span::from("Save")]
        )
      ).centered(),
      lyrics,
      index,
    }
  }
  fn select(&mut self, new: SelectableItem) {
    self.selection = new;
    self.lang_input.focus(self.selection == SelectableItem::Lang);
    self.lang_input.focus(self.selection == SelectableItem::Lang);
    self.desc_input.focus(self.selection == SelectableItem::Desc);
    self.text_input.focus(self.selection == SelectableItem::Text);
  }
}

impl Screen for LyricsScreen {
  fn draw(&mut self, frame: &mut Frame, app: &State) {
    let [header_area, lang_area, desc_area, text_area, footer_area] = Layout::vertical(
      vec![
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1)
      ]
    ).areas(frame.area());

    frame.render_widget(&self.header_par, header_area);
    frame.render_widget(&self.lang_input, lang_area);
    frame.render_widget(&self.desc_input, desc_area);
    frame.render_widget(&self.text_input, text_area);
    frame.render_widget(&self.footer_par, footer_area);
  }
  fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut State) -> Option<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Up, KeyModifiers::CONTROL) => {
        self.select(match self.selection {
          SelectableItem::Lang => SelectableItem::Text,
          SelectableItem::Desc => SelectableItem::Lang,
          SelectableItem::Text => SelectableItem::Desc,
        });
        return None;
      }
      (KeyCode::Down, KeyModifiers::CONTROL) => {
        self.select(match self.selection {
          SelectableItem::Lang => SelectableItem::Desc,
          SelectableItem::Desc => SelectableItem::Text,
          SelectableItem::Text => SelectableItem::Lang,
        });
        return None;
      }
      (KeyCode::Char('s' | 'ы'), KeyModifiers::CONTROL) => {
        Some(UiCommand::ChangeScreen(ui_enums::ScreenKind::Home))
      }
      _ => {
        match self.selection {
          SelectableItem::Lang => {
            if self.lang_input.input(key_event) {
              self.lyrics.lang.edit(self.lang_input.first_line_text());
              self.lang_input.highlight_content(self.lyrics.lang.changed());
            }
          }
          SelectableItem::Desc => {
            if self.desc_input.input(key_event) {
              self.lyrics.desc.edit(self.desc_input.first_line_text());
              self.desc_input.highlight_content(self.lyrics.desc.changed());
            }
          }
          SelectableItem::Text => {
            if self.text_input.input(key_event) {
              self.lyrics.text.edit(self.text_input.first_line_text());
              self.text_input.highlight_content(self.lyrics.text.changed());
            }
          }
        }
        return None;
      }
    }
  }
}
