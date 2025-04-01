use ratatui::{ style::{ Color, Style, Styled, Stylize }, widgets::Block };
use crate::ui::{ WidgetState };

pub trait BlockTrait {
  fn state_styled(&self, state: WidgetState) -> Self;
}

impl BlockTrait for Block<'_> {
  fn state_styled(&self, state: WidgetState) -> Self {
    Block::from(self.clone()).title_style(Style::from(state)).border_style(Style::from(state))
  }
}
