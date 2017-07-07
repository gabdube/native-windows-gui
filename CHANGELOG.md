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
* Events are no longer restrained on controls. This means that it is now possible to extend builtin controls with custom user events!
* Uis no longuer implictly free the children when unpacking a control. Instead the children handles
  must be returned in a `Vec` by the `Control.children` method. This method can be ignored if the
  control can't have children.
* Reworked the `Canvas` control internals. Now the canvas resource are simple NWG resources.
For more information also see: 
  * https://gabdube.github.io/native-windows-gui/book/canvas.html (canvas basics)
* The `Canvas` control is now feature gated behind the feature `canvas`. This is because `d2d1.lib` is not included with the gnu version
  and requires a few extra (annoying) steps. 

## New resources and controls

* The **Image** resource to load bitmap, ico and cursor files
* The **ImageFrame** control to display a bitmap in a window
* The **Frame** control to display a bordered frame inside another window
* The **TreeView** control. To display tree hierarchy of data
* The **TreeViewItem** control. To display an item in a tree view control
* The **ContextMenu** control. A pop-up menu that can be shown anywhere in screen. usually pops when the user right click the mouse.


## Existsing control changes

* **Window**
    * `set_icon` and `get_icon`: Allow the user to set or get the window icon  

* **Frame**
  * No longuer use the default static control. Allow more flexibility.

* **UI**
  * `has_handle`: Check if the ui has an object identified by an handle
  * `id_from_handle`: Return the `ID` associated with an HANDLE
  * `type_of_control`: Return the `ControlType` associated with a control

* **Most controls**
  * `set_font` and `get_font`: Allow the user to set the font or get the font identifier of many built-in controls  
  * `update`: Force a control to redraw itself.  
  * `focus`: Sets the keyboard focus to the specified control


## Other methods

* `toggle_console`: Hide or show the program console
* The `Cursor` struct. A fieldless struct to interface over the system cursor.
    * `get_position`: Return the cursor position in the screen
    * `set_position`: Set the cursor position in the screen
    * `get`: Set the cursor look (using a Cursor Image resource)
    * `set`: Get the cursor resource identifier (using a Cursor Image resource)
    * `get_capture`: Capture the mouse for a control
    * `set_capture`: Get the identifier of the control that captures the mouse
    * `release`: Release the mouse capture set with `set_capture`
    * `dragging`: Check if the user tries to drag a control. Mostly used with a `set_capture`/`release` pair   


# BETA 1 (0.2.0)

Initial beta release. This is a complete rewrite of the ALPHA, so there's nothing to list