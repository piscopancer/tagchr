// use ratatui::widgets::{ Block, Table };
// use super::{ block::BlockTrait, widget::FocusableWidget };

// #[derive(Clone)]
// pub struct StatefulTable {
//   init_block: Block<'static>,
//   pub table: Table<'static>,
//   enabled: bool,
// }

// impl StatefulTable {
//   pub fn new(table: Table<'static>, backup_block: Block<'static>) -> Self {
//     Self {
//       enabled: true,
//       table,
//       init_block: backup_block,
//     }
//   }
// }

// impl FocusableWidget for StatefulTable {
//   fn focus(&mut self, v: bool) {
//     self.table = self.table.clone().block(self.init_block.state_styled(v));
//   }
//   fn focused(&mut self, v: bool) -> Self {
//     self.focus(v);
//     self.clone()
//   }
// }
