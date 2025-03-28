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
  while app.state.running {
    app.poll();
  }
  ratatui::restore();
}
