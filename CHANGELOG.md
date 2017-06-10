# BETA 2 (0.3.0)

* **Breaking!** New way to handle events. Allow custom event definition and improve the event dispatching loop.  
For example, instead of `nwg::Event::Click`, use `nwg::events::button::Click`. 
For more information also see: 
  * https://gabdube.github.io/native-windows-gui/book/events.html (events basics)
  * https://gabdube.github.io/native-windows-gui/book/custom_events.html (defining custom events)
  * https://github.com/gabdube/native-windows-gui/blob/master/examples/templating.rs (simple example)
* New resource:
  * The **Image** resource to load bitmap, ico and cursor files
* New controls:
  * The **ImageFrame** control to display a bitmap in a window
* New methods for `UI`:
  * `has_handle`: Check if the ui has an object identified by an handle
  * `id_from_handle`: Return the `ID` associated with an HANDLE
* New methods:
  * `toggle_console`: Hide or show the program console


# BETA 1 (0.2.0)

Initial beta release. This is a complete rewrite of the ALPHA, so there's nothing to list