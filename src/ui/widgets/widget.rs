pub trait FocusableWidget {
  fn focus(&mut self, v: bool);
  fn focused(&mut self, v: bool) -> Self;
}

pub trait ToggleableWidget {
  fn toggle(&mut self, v: bool);
  // fn toggled(&mut self, v: bool) -> Self;
}

pub trait WidgetWithEditableContent {
  fn highlight_content(&mut self, v: bool);
}

pub trait TagWidget: FocusableWidget + WidgetWithEditableContent + ToggleableWidget {
  // Can add tag-specific methods here
  // fn tag_operation(&self) {
  //   self.highlight_border();
  //   self.highlight_content();
  // }
}

impl<T> TagWidget for T where T: FocusableWidget + WidgetWithEditableContent + ToggleableWidget {}
