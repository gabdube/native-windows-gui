/*!
    Macro based template system.
*/

/*
    Copyright (C) 2016  Gabriel Dub�

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

/**
    Return controls from the Ui. Panics if one of the controls could not be retrieved.  
    
    
    To avoid the panic, use `ui.get` directly.

    Usage:  
    `let control = nwg_get!(ui; (control ID, control type))`  
    `let (control1, control2) = nwg_get!(ui; [(control1_ID, control1 type), (control2_ID, control2_type)])`  
*/
#[macro_export]
macro_rules! nwg_get {
    ( $ui:ident; ($n:expr, $t:ty) ) => {
        $ui.get::<$t>(&$n).expect("Failed to find a control")
    };

    ( $ui:ident; [ $( ($n:expr, $t:ty) ),* ] ) => {
        (
            $( $ui.get::<$t>(&$n).expect("Failed to find a control") ),*
        )
    }
}

/**
    Return controls from the Ui. Panics if one of the controls could not be retrieved.  
    
    
    To avoid the panic, use `ui.get` directly. This is the mutable version of `nwg_get!`

    Usage:  
    `let control = nwg_get_mut!(ui; (control ID, control type))`  
    `let (control1, control2) = nwg_get_mut!(ui; [(control1_ID, control1 type), (control2_ID, control2_type)])`  
*/
#[macro_export]
macro_rules! nwg_get_mut {
    ( $ui:ident; ($n:expr, $t:ty) ) => {
        $ui.get_mut::<$t>(&$n).expect("Failed to find a control")
    };

    ( $ui:ident; [ $( ($n:expr, $t:ty) ),* ] ) => {
        (
            $( $ui.get_mut::<$t>(&$n).expect("Failed to find a control") ),*
        )
    }
}

/**
    Generates a function that initialize the content of a UI. 

    **head**: Define the name of the function and Ui key type. Ex: `my_function<my_type>`

    **controls**: A list of controls to add to the Ui. Accepts an array of `(ID, Template)`

    **events**: A list of events to bind to the controls of the UI. Accepts an array of `(ControlID, EventID, Event, Callback)`

    **resources**: A list of resources to add to the Ui. Accepts an array of `(ID, Template)`

    **values**: A list of user values to add to the Ui. Accepts an array of `(ID, Template)`

    **Return Value**: The function calls `commit` before returning. If the Ui initialization fails,
    the function will return the error.

    Usage: 

    ```rust  
       #[macro_use] extern crate native_windows_gui as nwg;

        nwg_template!(
            head: setup_ui2<&'static str>,
            controls: [ 
                ("MainWindow", nwg_window!( title="Template Example"; size=(280, 105) ))
            ];
            events: [ 
                ("MainWindow", "ACTION", nwg::events::Resized, |ui, caller, event, args| { 
                    println!("Hello World!"); 
                })
            ];
            resources: [ 
                ("MainFont", nwg_font!(family="Arial"; size=27)),
                ("TextFont", nwg_font!(family="Arial"; size=17))
            ];
            values: [ ("True", true), ("()", ()) ]
        ); 

        fn main() {}

    ```

*/
#[macro_export]
macro_rules! nwg_template {
    (
        head: $n:ident<$t:ty>,
        controls: [ $( ($i1:expr, $c:expr) ),* ];
        events: [ $( ($i4:expr, $i5:expr, $e:expr, $b:expr) ),*  ];
        resources: [ $( ($i2:expr, $r:expr) ),* ];
        values: [ $( ($i3:expr, $v:expr) ),* ]
    ) => 
    {

pub fn $n(ui: &$crate::Ui<$t>) -> Result<(), $crate::Error> {

    $( ui.pack_value(&$i3, $v); );*

    $( ui.pack_resource(&$i2, $r); );*

    $( ui.pack_control(&$i1, $c); );*
    $( ui.bind(&$i4, &$i5, $e, $b); );*
   
    ui.commit()
}
       
    }
}

//---- Controls ----//

/**
    Sane defaults for the Window control.

    Defaults:  
    • title: `"Native Windows GUI"`  
    • position: `(100, 100)`  
    • size: `(800, 600)`  
    • resizable: `false`  
    • visible: `true`  
    • disabled: `false`  
    • exit_on_close: `true`  
    • icon: `None`  

    Usage:  
    `nwg_window!()`  
    `nwg_window!(visible=false; resizable=true)`  
    `nwg_window!(\* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_window {
    ( $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::WindowT{ 
            title: "Native Windows GUI", 
            position: (100, 100), size: (800, 600), 
            resizable: false, visible: true, disabled: false, 
            exit_on_close: true, icon: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the Button control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • font: `None`

    Usage:  
    `nwg_button!(parent="MyParent";)`  
    `nwg_button!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_button!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_button {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ButtonT{ 
            text: "", 
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the CheckBox control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • checkstate: `CheckState::Unchecked`  
    • tristate: `false`  
    • font: `None`

    Usage:  
    `nwg_checkbox!(parent="MyParent";)`  
    `nwg_checkbox!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_checkbox!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_checkbox {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::CheckBoxT{ 
            text: "", 
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            checkstate: $crate::constants::CheckState::Unchecked,
            tristate: false,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the Combobox control. Requires a parent.

    `Data` parameter can be ommited if a default collection is passed.

    Defaults:  
    • collection: `[]`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • placeholder: `None`  
    • font: `None`

    Usage:  
    `nwg_combobox!(data=String; parent="MyParent";)`  
    `nwg_combobox!(parent="MyParent"; visible=false; collection=vec!["TEST", "TEST2"])`  
    `nwg_combobox!(data=&'static str; parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_combobox {
    (data=$t:ty, parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ComboBoxT::<$t>{ 
            collection: [],
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            placeholder: None,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }};
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ComboBoxT{ 
            collection: vec![],
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            placeholder: None,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the Label control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • align: `HTextAlign::Left`  
    • font: `None`

    Usage:  
    `nwg_label!(parent="MyParent";)`  
    `nwg_label!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_label!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_label {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::LabelT{ 
            text: "",
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            align: $crate::constants::HTextAlign::Left,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the ListBox control. Requires a parent.

    `Data` parameter can be ommited if a default collection is passed.

    Defaults:  
    • collection: `[]`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • readonly: `false`  
    • multi_select: `false`  
    • font: `None`

    Usage:  
    `nwg_listbox!(parent="MyParent";)`  
    `nwg_listbox!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_listbox!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_listbox {
    (data=$t:ty; parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ListBoxT::<$t, _>{ 
            collection: vec![],
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, readonly: false, multi_select: false,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }};

    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ListBoxT::<_, _>{ 
            collection: vec![],
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, readonly: false, multi_select: false,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}


/**
    Sane defaults for the Menu control. Requires a window parent.

    Defaults:  
    • text: `"Menu"`  
    • disabled: `false`  

    Usage:  
    `nwg_menu!(parent="MyParent";)`  
    `nwg_menu!(parent="MyParent"; text="AAA")`  
    `nwg_menu!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_menu {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::MenuT{  text: "Menu", parent: $p, disabled: false  };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the MenuItem control. Requires a menu parent.

    Defaults:  
    • text: `"Menuitem"`  
    • disabled: `false`  

    Usage:  
    `nwg_menuitem!(parent="MyParent";)`  
    `nwg_menuitem!(parent="MyParent"; text="AAA")`  
    `nwg_menuitem!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_menuitem {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t =  $crate::MenuItemT{  text: "Menuitem", parent: $p, disabled: false };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the Separator control. Requires a menu parent.

    The separator control do not have any properties beside the required parent.

    Usage:  
    `nwg_separator!(parent="MyParent";)`   
*/
#[macro_export]
macro_rules! nwg_separator {
    (parent=$p:expr) => { {
        $crate::SeparatorT{ parent: $p }
    }}
}

/**
    Sane defaults for the RadioButton control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • checkstate: `CheckState::Unchecked`  
    • font: `None`

    Usage:  
    `nwg_radiobutton!(parent="MyParent";)`  
    `nwg_radiobutton!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_radiobutton!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_radiobutton {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::RadioButtonT{
            text: "",
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            parent: $p,
            checkstate: $crate::constants::CheckState::Unchecked,
            font: None
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the Timer control.

    Defaults:  
    • interval: `1000` (1 second)

    Usage:  
    `nwg_timer!(parent="MyParent";)`  
    `nwg_timer!(parent="MyParent"; interval=1)`  
*/
#[macro_export]
macro_rules! nwg_timer {
    ($( $i:ident=$v:expr );*) => { {
        let mut t = 
        $crate::TimerT{
            interval: 1000
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the TextInput control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • readonly: `false`  
    • password: `false`  
    • limit: `32_767`  
    • placeholder: `None`  
    • font: `None`

    Usage:  
    `nwg_textinput!(parent="MyParent";)`  
    `nwg_textinput!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_textinput!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_textinput {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::TextInputT::<_, &'static str, _> {
            text: "",
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, readonly: false, password: false,
            limit: 32_767,
            placeholder: None,
            parent: $p,
            font: None
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the TextBox control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • readonly: `false`  
    • limit: `32_767`  
    • scrollbars: `(false, false)`  
    • font: `None`

    Usage:  
    `nwg_textbox!(parent="MyParent";)`  
    `nwg_textbox!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_textbox!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_textbox {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::TextBoxT::<_, _> {
            text: "",
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, readonly: false,
            limit: 32_767,
            scrollbars: (false, false),
            parent: $p,
            font: None
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the GroupBox control. Requires a parent.

    Defaults:  
    • text: `""`  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • align: `HTextAlign::Left`  
    • font: `None`

    Usage:  
    `nwg_groupbox!(parent="MyParent";)`  
    `nwg_groupbox!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_groupbox!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_groupbox {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::GroupBoxT {
            text: "",
            position: (0, 0), size: (100, 100), 
            visible: true, disabled: false,
            align: $crate::constants::HTextAlign::Left,
            parent: $p,
            font: None
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the ProgressBar control. Requires a parent.

    Defaults:  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • range: `(0, 100)`  
    • step: `10`  
    • value: `0`  
    • state: `ProgressBarState::Normal`  
    • vertical: `false`  
    • font: `None`

    Usage:  
    `nwg_progressbar!(parent="MyParent";)`  
    `nwg_progressbar!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_progressbar!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_progressbar {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ProgressBarT {
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false,
            range: (0, 100),
            step: 10,
            value: 0,
            state: $crate::constants::ProgressBarState::Normal,
            vertical: false,
            parent: $p,
        };
        $( t.$i = $v; );*
        t
    }} 
}

/**
    Sane defaults for the DatePicker control. Requires a parent.

    Defaults:  
    • value: Todays date
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  
    • font: `None`  
    • align: `HTextAlign::Left`  
    • format: The system locale format in a short format (ex: 2017-01-01)  
    • optional: `false`  

    Usage:  
    `nwg_DatePicker!(parent="MyParent";)`  
    `nwg_DatePicker!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_DatePicker!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_datepicker {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::DatePickerT{ 
            value: None,
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            align: $crate::constants::HTextAlign::Left,
            parent: $p, font: None,
            format: "", optional: false,
            range: (None, None)
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the FileDialog control.

    Defaults:  
    • parent: `None`  
    • title: `"Open file"`  
    • action: `FileDialogAction::Open`  
    • multiselect: `false`  
    • default_folder: `None`  
    • filters: `None`  

    Usage:  
    `nwg_filedialog!()`  
    `nwg_filedialog!(parent="MyParent"; title="Hey buddy!")`  
    `nwg_filedialog!(\* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_filedialog {
    ($( $i:ident=$v:expr );*) => { {
        let mut t = 
        $crate::FileDialogT::<_, _>{ 
            parent: None,
            title: "Open file",
            action: $crate::constants::FileDialogAction::Open,
            multiselect: false,
            default_folder: None,
            filters: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the Canvas control. Requires a parent.

    Defaults:  
    • position: `(0, 0)`  
    • size: `(100, 30)`  
    • visible: `true`  
    • disabled: `false`  

    Usage:  
    `nwg_canvas!(parent="MyParent";)`  
    `nwg_canvas!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_canvas!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_canvas {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::CanvasT {
            position: (0, 0), size: (100, 100), 
            visible: true, disabled: false,
            parent: $p,
        };
        $( t.$i = $v; );*
        t
    }}
}

/**
    Sane defaults for the ImageFrame control. Requires a parent.

    Defaults:  
    • position: `(0, 0)`  
    • size: `(100, 100)`  
    • visible: `true`  
    • disabled: `false`  
    • image: `None`  

    Usage:  
    `nwg_image_frame!(parent="MyParent";)`  
    `nwg_image_frame!(parent="MyParent"; visible=false; size=(10, 10))`  
    `nwg_image_frame!(parent="MyParent"; \* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_image_frame {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::ImageFrameT{ 
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            parent: $p, image: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}


//---- Resources ----//

/**
    Sane defaults for the Font resource.

    Defaults:  
    • family: `"Arial"`  
    • size: `12`  
    • weight: `FONT_WEIGHT_NORMAL`  
    • decoration: `FONT_DECO_NORMAL`  

    Usage:  
    `nwg_font!()`  
    `nwg_font!(family="Comic Sans")`  
    `nwg_font!(\* Any combinations of the template properties*\)`    
*/
#[macro_export]
macro_rules! nwg_font {
    ($( $i:ident=$v:expr );*) => { {
        let mut t = 
        $crate::FontT{ 
            family: "Arial", size: 12,
            weight: $crate::constants::FONT_WEIGHT_NORMAL,
            decoration: $crate::constants::FONT_DECO_NORMAL,
        };
        
        $( t.$i = $v; );*

        t
    }}
}

/**
    Sane defaults for the Image resource.  
    The `source` attribute is required.

    Defaults:  
    • image_type: `ImageType::Bitmap`  
    • size: `(0,0)`  

    Usage:  
    `nwg_image!(source="test.bmp")`  
    `nwg_image!(source="test.ico"; image_type=ImageType::Icon)`   
*/
#[macro_export]
macro_rules! nwg_image {
    (source=$s:expr; $( $i:ident=$v:expr );*) => { {
        let mut t = 
        $crate::ImageT{ 
            source: $s,
            strict: false,
            image_type: $crate::constants::ImageType::Bitmap,
            size: (0, 0)
        };
        
        $( t.$i = $v; );*

        t
    }}
}