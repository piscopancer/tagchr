<!--
has a magic space in-between
 s
 s - bad normal whitespace
 s
 dir
-->

<!--
SelectableWidget (for highlighting border)
WidgetWithEditableContent (for highlighting text/elements inside)
TagWidget = SelectableWidget + WidgetWithEditableContent

use cases

table (list of mp3 files) only needs SelectableWidget bcs there's no tag to edit
inputs need TagWidget to highlight borders when selected and to highlight text when tag changed
lyrics button needs TagWidget and it will highlight "edit" when lyrics tag is changed
-->

# todo

- differentiate between `Input` and `TextArea`, maybe using builder pattern on `TextArea` - `.single_line()`.
- those inputs that implement selectable must hide cursos when not selected alongside highlighting borders
<!-- - impl iter next and prev logic for selectable elements enums (cannot do it on selectable enums themselfves, may wrap in a struct `Selection`) -->
- use modules to encapsulate movement between sections. will be useful for side-effects (when ctrl-down into table should also put text into inputs, cannot change sections just by modifying app state bcs it's not enough)
- if input text was highlighted (meaning changed happened), selecting another song does not reset it
- search bar not working
- use hashmap to store screens by enum and generic trait (for unwrapping during compilation)
<!-- - (maybe) `TagInput(TextArea)` that changes tag that does field editing and highlighting on its own to avoid duplication -->
- after selecting another song with arrow keys, text highlight should be auto removed
- trait `TagWidget` and structs `TagInput`/`TagButton` (for lyrics), still too generic, idk if i need this abstraction

<!-- -->

# ideas

## editable tags

editable tag can itself be versioned if string, meaning it has original state and edited state. should make a `Editable` trait/struct. year is editable tag, lyrics is a tag too, however it has 3 fields which are editable, while year is itself editable.

## screen toggling

for home screen need a way to init with preselected editor i, so that i can go back to it from lyrics screen. lyrics would hold and provide the i when moving back, also using state finding the mp3 tag and updating it with edited lyrics. may create a separate struct called router that implements relations for every screen struct with turbo fish or match, match would be the easiest though, maybe use tuples for that (prev, next)

## selectable items

with a director of selectable items it's possible to remove highlight on all and add on new

## splitting ui into groups

because miltiple screens can show same app data, there should be no 1 to 1 relation between screen/section/etc and data, e.g. HomeScreen and HomeData makes no sense. providing enums with data like HomeScreen(HomeData) makes no sense either.

screens are just pages in web. only one screen can appear on the screen. screens have sections which they are split with. for example, main screen has 3 sections: path, edit, list. screen is created with a section currently selected. section handles key strokes differently so active section is given a key event.

screens and sections must be represented as enums and structs. enums would be used for navigation while structs hold actual data
