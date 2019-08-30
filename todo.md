# Main Features

## Control List

 [] Button
 [] ComboBox
 [] DateTime
 [] ListBox
 [] ListView
 [] MonthCalendar
 [] ProgressBar
 [] RichEdit (includes LineEdit)
 [] Static
 [] SysLink
 [] Tabs
 [] ToolBar
 [] MenuBar
 [] ToolTip
 [] TreeView
 [] FileDialog

## Controls that must be manually reimplemented

 [] Number select (UpDown)
 [] GroupBox
 [] Frame

## Actions

 [] Add built-in control to the interface
 [] Remove built-in control to the interface AT RUNTIME
 [] Add composited user control to the interface (see Composition)

## Events

 [] Allow users to associate one or more callback to a control event
 [] Allow users to add event callbacks at runtime
 [] Allow users to remove event callbacks at runtime

## Layout

 [] HBoxLayout
 [] VBoxLayout
 [] GridLayout
 [] FormLayout

## Composition

 [] Allow base control to be referenced in a user struct
 [] Allow user structs with UI controls to be referenced in other user structs

## Derive

 [] Automate children control instantiation using a custom derive
 [] Define the event callbacks in the derive
 [] Define the layout in the derive
