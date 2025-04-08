use ratatui::{ style::{ Color, Style, Styled, Stylize }, widgets::Block };
use crate::ui::{ StyleFlags };

pub trait BlockTrait {
  fn state_styled(&self, state: StyleFlags) -> Self;
}

impl BlockTrait for Block<'_> {
  fn state_styled(&self, state: StyleFlags) -> Self {
    Block::from(self.clone()).title_style(Style::from(state)).border_style(Style::from(state))
  }
}
