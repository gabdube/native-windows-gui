1.0.12
* A new plotting control
* Added support for system key events  (thanks to dnlmlr)
* Replace target_arch with target_pointer_width (thanks to skyfloogle)
* Minor improvements in the rich label control
* Fix window centering with high dpi enabled
* New release for native-windows-derive (1.0.4)
  * Support for generics in native-windows-derive (thanks to RicoRodriges)
  * Fix deriving partial into into other partials (thanks to yakov-bakhmatov)

1.0.11
* Double buffer option for ListView
* Fix a treeview issue when building without the image-list feature
* FIx dpi awareness with min/max functions
* Deprecate `Timer`
* Added `AnimationTimer`, a all around better timer component
* Fix some resource leak
* Added support for `raw-window-handle` ini Window and ExternCanvas
* Fixed to panics warnings on rust 1.51.0

1.0.10
* Fixed a compiling bug when using `no-default-features`

1.0.9
* BREAKING CHANGE: File dialog `get_selected_item` & `get_selected_items` now return `OsString` instead of String to handle some exotic Windows path
* Added vertical alignment  `v_align` to label (defaults to center)
* Added vertical alignment `v_align` to combobox (defaults to center)
* Fixed multi-line label alignment 
* Added helpers functions to Bitmap and Icons
* Added the `Monitor` struct to query monitor and screen information
* Added `center` to the window builder to center a window on screen
* Fixed the text input refresh after calling `set_password_char`
* `maximise`, `minimise`, `restore` for window control

1.0.8

* Added `Menu::popup_with_flags` to customize the display of popup menus
* Added a way to specify the extended window flags to all controls
* Added a new layout type `Dynamic Layout` (thanks to RicoRodriges)
* Added a way to directly set the column width in a data grid view (thanks to RicoRodriges)
* Some grammar and spelling fix (thanks to bingen13 and celialewis3)
* Fix the derive macro so that it works with the rust 2018 module aliasing (native-windows-derive v 1.0.3)

1.0.7

* Fixed support for the GNU toolchain

1.0.6

* Added rich textbox feature
* Added rich label control (a label using tich text box under the hood)
* Fixed the headers of the list view and few other rendering bug (thanks to RicoRodriges)
* New list view events (thanks to RicoRodriges)
* Fixed a severe memory leak in image creation (thanks to RicoRodriges)
* Fix a z order bug with the tab naviguation
* Added `selected_item_count` and `selected_items` to Treeview.
* Fixed `selected_item` in TreeView to support item set programatically

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