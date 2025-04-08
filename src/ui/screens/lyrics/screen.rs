use std::sync::mpsc::Sender;
use crate::{
  app::{ app::{ App, Command }, state::State, tag::LyricsEditableTag },
  ui::{
    block::BlockTrait,
    home::{ self, screen::{ EditorFocusable, HomeScreen } },
    shortcut::Shortcut,
    text_area::TextAreaTrait,
    ui_enums::{ self, Screen },
    InputHandler,
    StateDependentWidget,
    StringTrait,
    UiState,
    StyleFlags,
  },
};
use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use id3::frame::Lyrics;
use ratatui::{
  buffer::Buffer,
  layout::{ Constraint, Layout, Margin, Rect },
  style::{ Color, Style, Stylize },
  text::{ Line, Span },
  widgets::{ Block, BorderType, Paragraph, Widget },
  Frame,
};
use tui_textarea::TextArea;

#[derive(Clone, Copy, PartialEq)]
pub enum Focusable {
  Lang,
  Desc,
  Text,
}

pub struct LyricsScreen {
  // TODO: file name
  pub index: usize,
  pub lyrics: LyricsEditableTag,
  pub focused_el: Focusable,
  pub lang_input: TextArea<'static>,
  pub desc_input: TextArea<'static>,
  pub text_textarea: TextArea<'static>,
}

impl LyricsScreen {
  pub fn new(index: usize, lyrics: LyricsEditableTag) -> Self {
    let focused_el = Focusable::Lang;
    Self {
      lang_input: {
        let mut input = TextArea::new(Vec::from([lyrics.lang.to_string()]));
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Lang"));
        input.set_cursor_line_style(Style::new());

        input
      },
      desc_input: {
        let mut input = TextArea::new(Vec::from([lyrics.desc.to_string()]));
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Desc"));
        input.set_cursor_line_style(Style::new());

        input
      },
      text_textarea: {
        let mut input = TextArea::new(Vec::from([lyrics.text.to_string()]));
        input.set_block(Block::bordered().border_type(BorderType::Rounded).title_top("Text"));
        input.set_cursor_line_style(Style::new());

        input
      },
      focused_el,
      lyrics,
      index,
    }
  }
}

impl InputHandler for LyricsScreen {
  fn handle_input(
    &self,
    state: &State,
    ui_state: &UiState,
    event: Event,
    sender: Sender<Command>
  ) -> bool {
    match event {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        match (key_event.code, key_event.modifiers, self.focused_el) {
          (KeyCode::Esc, _, _) => {
            sender.send(
              Command::SetScreen(
                Screen::Home(
                  HomeScreen::new(
                    home::screen::Focusable::Editor(self.index, EditorFocusable::LyricsButton),
                    // TODO: fix this shit, what does it do here? why does home screen need reference to tags? it makes sense that it's used by `new` to populate inputs on the start but what if every screen had a function `on_start` that would expose state from which it could be possible to populate the shit? that way screen could be responsible for requiring data for population instead of expecting the screen changer to provide tags for it (exactly what happens here)
                    Some({
                      let tags = &state.get_file(self.index).tags;
                      tags
                    })
                  )
                )
              )
            );
            true
          }
          (KeyCode::Down, KeyModifiers::CONTROL, f_el) | (KeyCode::PageDown, _, f_el) => {
            sender.send(
              Command::FocusLyricsElement(match f_el {
                Focusable::Lang => Focusable::Desc,
                Focusable::Desc => Focusable::Text,
                Focusable::Text => Focusable::Lang,
              })
            );
            true
          }
          (KeyCode::Up, KeyModifiers::CONTROL, f_el) | (KeyCode::PageUp, _, f_el) => {
            sender.send(
              Command::FocusLyricsElement(match f_el {
                Focusable::Lang => Focusable::Text,
                Focusable::Desc => Focusable::Lang,
                Focusable::Text => Focusable::Desc,
              })
            );
            true
          }
          (KeyCode::Char('r' | 'к'), KeyModifiers::CONTROL, f_el) => {
            sender.send(Command::ResetLyricsScreenTag(f_el));
            true
          }
          _ => {
            sender.send(Command::HandleLyricsScreenInput(key_event.clone(), self.focused_el));
            true
          }
          _ => false,
        }
      }
      _ => false,
    }
  }
}

impl StateDependentWidget for LyricsScreen {
  fn render_from_state(&self, area: Rect, buf: &mut Buffer, state: &State, ui_state: &UiState)
    where Self: Sized
  {
    let [header_area, lang_area, desc_area, text_area, footer_area] = Layout::vertical(
      vec![
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1)
      ]
    ).areas(area);
    let footer_area = footer_area.inner(Margin::new(1, 0));

    let header_par = Paragraph::new(vec![Line::from("Lyrics Editing"), Line::from("").gray()])
      .centered()
      .render(header_area, buf);

    let tags = &state.get_file(self.index).tags;

    {
      let mut lang_input = self.lang_input.clone();
      let border_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: self.focused_el == Focusable::Lang,
      };
      let text_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: tags.lyrics.lang.edited(),
      };
      lang_input.set_style(Style::from(text_flags));
      lang_input.set_block(
        lang_input.block().cloned().unwrap_or_default().border_style(Style::from(border_flags))
      );
      lang_input.toggle_cursor(border_flags.highlighted);
      lang_input.render(lang_area, buf);
    }

    {
      let mut desc_input = self.desc_input.clone();
      let border_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: self.focused_el == Focusable::Desc,
      };
      let text_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: tags.lyrics.desc.edited(),
      };
      desc_input.set_style(Style::from(text_flags));
      desc_input.set_block(
        desc_input.block().cloned().unwrap_or_default().border_style(Style::from(border_flags))
      );
      desc_input.toggle_cursor(border_flags.highlighted);
      desc_input.render(desc_area, buf);
    }

    {
      let mut text_textarea = self.text_textarea.clone();
      let border_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: self.focused_el == Focusable::Text,
      };
      let text_flags = StyleFlags {
        enabled: true,
        valid: true,
        highlighted: tags.lyrics.text.edited(),
      };
      text_textarea.set_style(Style::from(text_flags));
      text_textarea.set_block(
        text_textarea.block().cloned().unwrap_or_default().border_style(Style::from(border_flags))
      );
      text_textarea.toggle_cursor(border_flags.highlighted);
      text_textarea.render(text_area, buf);
    }

    let footer_par = Paragraph::new(
      Line::from(
        [
          Shortcut::new("Esc", "Back", Color::Gray).to_spans(),
          Vec::from([Span::from(" :: ").dark_gray()]),
          Shortcut::new("Ctrl+R", "Reset field", Color::Gray).to_spans(),
        ].concat()
      )
    )
      .right_aligned()
      .render(footer_area, buf);
  }
}
