#![allow(warnings)]

use std::{ cell::RefCell, fs, path::{ Path, PathBuf }, sync::{ Arc, Mutex }, time::Duration };
use app::app::App;
use crossterm::event::{ self, poll, Event, KeyCode, KeyEvent, KeyEventKind };
use ratatui::Frame;
use ui::ui::Ui;

mod ui;
mod app;

fn main() {
  let mut app = App::new();
  let mut ui = Ui::new();

  let mut term = ratatui::init();

  loop {
    if !app.running {
      break;
    }
    if poll(Duration::from_millis(100)).unwrap() {
      match event::read().unwrap() {
        Event::Key(key_event) => if key_event.kind == KeyEventKind::Press {
          ui.handle_key_event(key_event, &mut app);
        }
        _ => {}
      }
      let _ = term
        .draw(|frame| ui.draw(frame, &app))
        .map_err(|_| {
          app.running = false;
        });
    }
  }

  ratatui::restore();
}
