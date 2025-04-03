use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use id3::frame::Lyrics;
use ratatui::{
  layout::{ Constraint, Layout },
  style::{ Color, Style, Stylize },
  text::{ Line, Span },
  widgets::{ Block, BorderType, Paragraph },
  Frame,
};
use tui_textarea::TextArea;
use crate::{
  app::{ app::{ App, State }, tag::LyricsEditableTag },
  ui::{
    block::BlockTrait,
    home::{ self, screen::HomeScreen },
    shortcut::Shortcut,
    text_area::TextAreaTrait,
    ui_enums,
    Screen,
    StringTrait,
    UiCommand,
    WidgetState,
  },
};

#[derive(Clone, Copy, PartialEq)]
enum FocusedElement {
  Lang,
  Desc,
  Text,
}

pub struct LyricsScreen {
  // file name
  // song total time or idk
  pub index: usize,
  pub lyrics: LyricsEditableTag,
  focused_el: FocusedElement,
  lang_input: TextArea<'static>,
  desc_input: TextArea<'static>,
  text_input: TextArea<'static>,
}

impl LyricsScreen {
  pub fn new(index: usize, lyrics: LyricsEditableTag) -> Self {
    let focused_el = FocusedElement::Lang;
    Self {
      lang_input: TextArea::new(vec![lyrics.lang.to_string()]),
      desc_input: TextArea::new(vec![lyrics.desc.to_string()]),
      text_input: TextArea::new(vec![lyrics.text.to_string()]),
      focused_el,
      lyrics,
      index,
    }
  }
}

impl Screen for LyricsScreen {
  fn draw(&mut self, frame: &mut Frame, state: &State) {
    let [header_area, lang_area, desc_area, text_area, footer_area] = Layout::vertical(
      vec![
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1)
      ]
    ).areas(frame.area());

    let header_par = Paragraph::new(
      vec![Line::from("Lyrics Editing"), Line::from("").gray()]
    ).centered();

    {
      self.lang_input.set_text(self.lyrics.lang.to_string());
      let selected = self.focused_el == FocusedElement::Lang;
      self.lang_input.toggle_cursor(selected);
      let ws = {
        let mut ws = WidgetState::empty();
        ws.insert(WidgetState::Enabled);
        ws.set(WidgetState::Highlighted, selected);
        ws
      };
      self.lang_input.set_block(
        Block::bordered().border_type(BorderType::Rounded).title_top("Language").state_styled(ws)
      );
      self.lang_input.set_style(
        Style::from({
          let mut ws = WidgetState::empty();
          ws.insert(WidgetState::Enabled);
          ws.set(WidgetState::Highlighted, self.lyrics.lang.edited());
          ws
        })
      );
    }

    {
      self.desc_input.set_text(self.lyrics.desc.to_string());
      let selected = self.focused_el == FocusedElement::Desc;
      self.desc_input.toggle_cursor(selected);
      let ws = {
        let mut ws = WidgetState::empty();
        ws.insert(WidgetState::Enabled);
        ws.set(WidgetState::Highlighted, selected);
        ws
      };
      self.desc_input.set_block(
        Block::bordered().border_type(BorderType::Rounded).title_top("Description").state_styled(ws)
      );
      self.desc_input.set_style(
        Style::from({
          let mut ws = WidgetState::empty();
          ws.insert(WidgetState::Enabled);
          ws.set(WidgetState::Highlighted, self.lyrics.desc.edited());
          ws
        })
      );
    }

    {
      self.text_input.set_text(self.lyrics.text.to_string());
      let selected = self.focused_el == FocusedElement::Text;
      self.text_input.toggle_cursor(selected);
      let ws = {
        let mut ws = WidgetState::empty();
        ws.insert(WidgetState::Enabled);
        ws.set(WidgetState::Highlighted, selected);
        ws
      };
      self.text_input.set_block(
        Block::bordered()
          .border_type(BorderType::Rounded)
          .title_top("Text (Lyrics)")
          .state_styled(ws)
      );
      self.text_input.set_style(
        Style::from({
          let mut ws = WidgetState::empty();
          ws.insert(WidgetState::Enabled);
          ws.set(WidgetState::Highlighted, self.lyrics.text.edited());
          ws
        })
      );
    }

    let footer_par = Paragraph::new(
      Line::from(
        [
          Shortcut::new("Esc", "Back", Color::Gray).to_spans(),
          Vec::from([Span::from(" :: ").dark_gray()]),
          Shortcut::new("Ctrl+R", "Reset field", Color::Gray).to_spans(),
        ].concat()
      )
    ).right_aligned();

    frame.render_widget(&header_par, header_area);
    frame.render_widget(&self.lang_input, lang_area);
    frame.render_widget(&self.desc_input, desc_area);
    frame.render_widget(&self.text_input, text_area);
    frame.render_widget(&footer_par, footer_area);
  }
  fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) -> Vec<UiCommand> {
    match (key_event.code, key_event.modifiers) {
      (KeyCode::Esc, _) => {
        return Vec::from([UiCommand::ChangeScreen(ui_enums::ScreenKind::Home)]);
      }
      (KeyCode::PageUp, _) | (KeyCode::Up, KeyModifiers::CONTROL) => {
        self.focused_el = match self.focused_el {
          FocusedElement::Lang => FocusedElement::Text,
          FocusedElement::Desc => FocusedElement::Lang,
          FocusedElement::Text => FocusedElement::Desc,
        };
      }
      (KeyCode::PageDown, _) | (KeyCode::Down, KeyModifiers::CONTROL) => {
        self.focused_el = match self.focused_el {
          FocusedElement::Lang => FocusedElement::Desc,
          FocusedElement::Desc => FocusedElement::Text,
          FocusedElement::Text => FocusedElement::Lang,
        };
      }
      (KeyCode::Char('r' | 'к'), KeyModifiers::CONTROL) => {
        match self.focused_el {
          FocusedElement::Lang => self.lyrics.lang.reset(),
          FocusedElement::Desc => self.lyrics.desc.reset(),
          FocusedElement::Text => self.lyrics.text.reset(),
        }
      }
      _ => {
        match self.focused_el {
          FocusedElement::Lang => {
            if self.lang_input.input(key_event) {
              self.lyrics.lang.edit(self.lang_input.text_as_single_line());
            }
          }
          FocusedElement::Desc => {
            if self.desc_input.input(key_event) {
              self.lyrics.desc.edit(self.desc_input.text_as_single_line());
            }
          }
          FocusedElement::Text => {
            if self.text_input.input(key_event) {
              self.lyrics.text.edit(self.text_input.lines().join("\n"));
            }
          }
        }
      }
    }
    Vec::new()
  }
}
