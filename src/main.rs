#![allow(warnings)]

use app::app::App;

mod ui;
mod app;
mod info;

fn main() {
  let mut app = App::new();
  while app.state.running {
    app.poll();
  }
  ratatui::restore();
}
