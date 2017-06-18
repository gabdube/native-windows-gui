# BETA 2 (0.3.0)

## Announcement

* License switch: GPLv3 to MIT 


## Breaking changes

* New way to handle events. Allow custom event definition and improve the event dispatching loop.  
For example, instead of `nwg::Event::Click`, use `nwg::events::button::Click`. 
For more information also see: 
  * https://gabdube.github.io/native-windows-gui/book/events.html (events basics)
  * https://gabdube.github.io/native-windows-gui/book/custom_events.html (defining custom events)
  * https://github.com/gabdube/native-windows-gui/blob/master/examples/templating.rs (simple example)
* Uis no longuer implictly free the children when unpacking a control. Instead the children handles
  must be returned in a `Vec` by the `Control.children` method. This method can be ignored if the
  control can't have children.


## New resources and controls

* The **Image** resource to load bitmap, ico and cursor files
* The **ImageFrame** control to display a bitmap in a window
* The **Frame** control to display a bordered frame inside another window
* The **TreeView** control. To display tree hierarchy of data
* The **TreeViewItem** control. To display an item in a tree view control


## Existsing control changes

* **Window**
    * `set_icon` and `get_icon`: Allow the user to set or get the window icon  
* **UI**
  * `has_handle`: Check if the ui has an object identified by an handle
  * `id_from_handle`: Return the `ID` associated with an HANDLE
  * `type_of_control`: Return the `ControlType` associated with a control

* **Most controls**
  * `set_font` and `get_font`: Allow the user to set the font or get the font identifier of many built-in controls  


## Other methods

* `toggle_console`: Hide or show the program console


# BETA 1 (0.2.0)

Initial beta release. This is a complete rewrite of the ALPHA, so there's nothing to list