// use ratatui::widgets::{ Block, Paragraph };

// use super::{ block::BlockTrait, widget::{ FocusableWidget, ToggleableWidget } };

// #[derive(Clone)]
// pub struct StatefulParagraph {
//   pub paragraph: Paragraph<'static>,
//   init_block: Block<'static>,
//   enabled: bool,
// }

// impl StatefulParagraph {
//   pub fn new(paragraph: Paragraph<'static>, backup_block: Block<'static>) -> Self {
//     Self { enabled: true, paragraph, init_block: backup_block }
//   }
// }

// impl ToggleableWidget for StatefulParagraph {
//   fn toggle(&mut self, v: bool) {
//     self.enabled = v;
//   }
// }

// impl FocusableWidget for StatefulParagraph {
//   fn focus(&mut self, v: bool) {
//     self.paragraph = self.paragraph.clone().block(self.init_block.state_styled(v));
//   }
//   fn focused(&mut self, v: bool) -> Self {
//     self.focus(v);
//     self.clone()
//   }
// }
