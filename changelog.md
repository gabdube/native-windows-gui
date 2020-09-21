1.0.5

* Added placeholder for textinput
* Fixed a crasher when resizing a tab container
* Icon can now be loaded from memory
* Bitmap can be converted to icons using `Bitmap::copy_as_icon`
* Systray pop up menu now closes when you click outside of them
* Embedded image loading functions (`EmbedResource::image`, `EmbedResource::icon`, etc) have a new `size` parameter
* 2 new events for menus
  * OnMenuEnter: Raised when a menu is shown on screen
  * OnMenuExit: Raised when a menu is closed by the user. Either by selecting an item or by clicking outside of it.

1.0.4

* Documentation fixes
* Get parent item in tree view
* Load any image from embed source
* Set/Get listview background color
* always show selected items in listview and treeview (even if the control does not have focus)

1.0.3

* A few bug fixes
* Add the ability to move controls in a grid layout
* Added the `OnMinMaxInfo` event to handle the min/max size of a window
* Fixed the tab header height

1.0.2

* Fix carriage return in multiline textbox. 100% sure it was working but heh
* Clipboard internal APi improvement by @DoumanAsh
* Update some progress bar flags at runtime with `add_flags` and `remove_flags` 
* Modal message boxes

1.0.1

* A quickfix for the docs.rs documentation.

1.0.0

* Initial release