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

- add help modal with ...
- display `save` shortcut and allow saving only if song was `edited`
- show in table using yellow if song has been `edited`
- singleton pattern for ui and state lol
- arrows in text areas not working
- maybe implement Widget/render for input
- use modules to encapsulate movement between sections. will be useful for side-effects (when ctrl-down into table should also put text into inputs, cannot change sections just by modifying app state bcs it's not enough)

<!-- -->

# ideas

## modals

- probably turn modals into enum and iterate over them to spawn
- different rects depending of the type (`Help`/`SaveSong`).
- must be different structs, `Help` handles does not have options so it does not handle arrow keys. `SaveSong` however has options (save/cancel) so it handles appropriate keys including arrows, s and c.
- trait `ModalWithOptions` with default impl for `option(option)`, `next`, `prev`
- modals are part of ui so they can modify state without any problem, so they must be provided with the `state` to mutate somehow

## event driven communication

state wants to recieve events from whatever elements are there below

## handling key events

reading key inputs is solely a feature of ui, not app state. however, match on key events must be done with `state` only, not `ui`, that's bcs `ui` is already in sync with `state` allowing to interact with the `state` in a way that `state` allows, if not, `ui` has issues

the idea is that key must be handled differently based on the ui state. normally i want different handling when there are different elements on the screen, which probably means it's better to match on ui state going with approach #0.

i currently consider these approaches

0. matching on screen and providing keys, basically (mainly) making screens -> sections -> elements responsible for handling key events. additionally read bools from ui state or screens to decide whether screen/section/element match should be triggered
1. matching on keys and then reading conditions using if else
2. return key event if it had no match so that the parent can handle it
3. firstly, key event is handled by blocking ui elements like popups, if . secondly, it's given to the current screen like home/lyrics. there's no other sitatuion.

- when pressing escape while popup is active, the latest popup (there will be a vec of them) is given keys events to handle. (specific impls of popup, btw). the popup, dep on its impl either closes or does nothing about it, leaving the screen unaware of the key event at all. blocking ui elements can appear anywhere despite current screen, they are global so they must act the first. i can have a separate vec of blocking els in this order of importance: popups -> toasts
- when lyrics screens is current, escape only returns to home screen, meanwhile escape on home exits the app.
- some actions will be more important than the other, so i need to use `match` and think of the order of matches + use `if` checks for additional check. for example when on lyrics screen, i should first check

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
