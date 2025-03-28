# todo

- use modules to encapsulate movement between sections. will be useful for side-effects (when ctrl-down into table should also put text into inputs, cannot change sections just by modifying app state bcs it's not enough)
- if input text was highlighted (meaning changed happened), selecting another song does not reset it
- search bar not working
- use hashmap to store screens by enum and generic trait (for unwrapping during compilation)
<!-- - (maybe) `TagInput(TextArea)` that changes tag that does field editing and highlighting on its own to avoid duplication -->
- after selecting another song with arrow keys, text highlight should be auto removed
- trait `TagWidget` and structs `TagInput`/`TagButton` (for lyrics), still too generic, idk if i need this abstraction

<!-- -->

<!--
screen: main
********** **********
*  path  * *  edit  *
********** *        *
********** *        *
*  list  * *        *
*        * *        *
*        * *        *
********** **********
-->

# ideas

## splitting ui into groups

because miltiple screens can show same app data, there should be no 1 to 1 relation between screen/section/etc and data, e.g. HomeScreen and HomeData makes no sense. providing enums with data like HomeScreen(HomeData) makes no sense either.

screens are just pages in web. only one screen can appear on the screen. screens have sections which they are split with. for example, main screen has 3 sections: path, edit, list. screen is created with a section currently selected. section handles key strokes differently so active section is given a key event.

screens and sections must be represented as enums and structs. enums would be used for navigation while structs hold actual data

# path

scans downloads and music dirs by default. search is used to sort out songs by name.

# table

displays

- name
- modified time (today, yesterday, this week, date)
- path

pressing enter while song is selected brings its data to the editor as currently edited song. changes are tracked in separate instance (same struct).

# edit

<!-- edited version only exists if currently edited song exists, meaning edits must be part of currently edited song -->

ctrl + s to save data
esc to remove currently edited song and return to table selection

(example with song name) when entering edit mode

1. set original name
2. set to input
3. set input's text to edit name

when changing text

1. set to input
2. set input's text to edit name

## checking for differences

need to have the original to reference for comparison
the edit should have identical shape
when edit field is None it means no edit was made to this field
when edit field is Some, it means it was modified
number fields?
