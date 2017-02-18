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

#[macro_export]
macro_rules! nwg_window {
    ( $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::WindowT{ 
            title: "Native Windows GUI", 
            position: (100, 100), size: (800, 600), 
            resizable: false, visible: true, disabled: false, 
            exit_on_close: true
        };
        
        $( t.$i = $v; );*

        t
    }}
}

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

#[macro_export]
macro_rules! nwg_label {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::LabelT{ 
            text: "A label",
            position: (0, 0), size: (100, 30), 
            visible: true, disabled: false, 
            align: $crate::constants::HTextAlign::Left,
            parent: $p, font: None
        };
        
        $( t.$i = $v; );*

        t
    }}
}

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


#[macro_export]
macro_rules! nwg_menu {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t = 
        $crate::MenuT{  text: "Menu", parent: $p, disabled: false  };
        $( t.$i = $v; );*
        t
    }}
}

#[macro_export]
macro_rules! nwg_menuitem {
    (parent=$p:expr; $( $i:ident=$v:expr );* ) => { {
        let mut t =  $crate::MenuItemT{  text: "Menuitem", parent: $p, disabled: false };
        $( t.$i = $v; );*
        t
    }}
}

#[macro_export]
macro_rules! nwg_separator {
    (parent=$p:expr) => { {
        $crate::SeparatorT{ parent: $p }
    }}
}

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


//---- Resources ----//

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