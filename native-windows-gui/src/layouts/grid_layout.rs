use crate::controls::ControlHandle;
use crate::win32::window::bind_raw_event_handler_inner;
use crate::win32::window_helper as wh;
use crate::NwgError;
use winapi::shared::windef::{HWND};
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;


/// A control item in a GridLayout
#[derive(Debug)]
pub struct GridLayoutItem {
    /// The handle to the control in the item
    control: HWND,

    /// The column position of the control in the layout
    pub col: u32,

    /// The row position of the control in the layout
    pub row: u32,

    /// The number column this item should span. Should be 1 for single column item.
    pub col_span: u32,

    /// The number row this item should span. Should be 1 for single row item.
    pub row_span: u32
}

impl GridLayoutItem {

    /// Initialize a new grid layout item
    pub fn new<W: Into<ControlHandle>>(c: W, col: u32, row: u32, col_span: u32, row_span: u32) -> GridLayoutItem {
        let control = c.into().hwnd().expect("Child must be a window-like control (HWND handle)");

        GridLayoutItem {
            control,
            col,
            row,
            col_span,
            row_span
        }
    }

}


/// A layout that lays out widgets in a grid
/// This is the inner data shared between the callback and the application
pub struct GridLayoutInner {
    /// The control that holds the layout
    base: HWND,

    /// The children of the control that fit in the layout
    children: Vec<GridLayoutItem>,

    /// The top, right, bottom, left space around the layout
    margins: [u32; 4],

    /// The minimum size of the layout. Used if `base` is smaller than `min_size`.
    min_size: [u32; 2],

    /// The maximum size of the layout. Used if `base` is bigger than `min_size`.
    max_size: [u32; 2],

    /// The number of column. If None, compute the value from children.
    column_count: Option<u32>,

    /// The number of row. If None, compute the value from children.
    row_count: Option<u32>, 

    /// The spacing between controls
    spacing: u32
}

/** 
A layout that lays out widgets in a grid
NWG layouts use interior mutability to manage their controls.

A GridLayouts has the following properties:
* margin - The top, right, bottom, left margins of the layout - (default: [5, 5, 5, 5])
* spacing - The spacing between children controls - (default: 5)
* min_size - The minimum size of the layout - (default: [0, 0])
* max_size - The maximum size of the layout - (default: [u32::max_value(), u32::max_value()])
* max_column - Number of columns - (default: None),
* max_row - Number of rows - (default: None),

```rust
    use native_windows_gui as nwg;
    fn layout(layout: &nwg::GridLayout, window: &nwg::Window, item1: &nwg::Button, item2: &nwg::Button) {
        nwg::GridLayout::builder()
            .parent(window)
            .max_row(Some(6))
            .spacing(5)
            .margin([0,0,0,0])
            .child(0, 0, item1)
            .child_item(nwg::GridLayoutItem::new(item2, 1, 0, 2, 1))
            .build(&layout);
    }
```
*/
#[derive(Clone)]
pub struct GridLayout {
    inner: Rc<RefCell<GridLayoutInner>>
}

impl GridLayout {

    pub fn builder() -> GridLayoutBuilder {
        let layout = GridLayoutInner {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [5, 5, 5, 5],
            spacing: 5,
            min_size: [0, 0],
            max_size: [u32::max_value(), u32::max_value()],
            column_count: None,
            row_count: None
        };

        GridLayoutBuilder { layout }
    }

    /**
        Add a children control to the grid layout. 
        This is a simplified interface over `add_child_item`
        
        Panic:
        - If the layout is not initialized
        - If the control is not window-like (HWND handle)
    */
    pub fn add_child<W: Into<ControlHandle>>(&self, col: u32, row: u32, c: W) {
        let h = c.into().hwnd().expect("Child must be a window-like control (HWND handle)");
        let item = GridLayoutItem {
            control: h,
            col,
            row,
            col_span: 1,
            row_span: 1,
        };

        self.add_child_item(item);
    }
    
    /** 
    Add a children control to the grid layout. 
    
    Panic:
        - If the layout is not initialized
        - If the control is not window-like (HWND handle)
    */
    pub fn add_child_item(&self, i: GridLayoutItem) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("GridLayout is not initialized");
            }

            // No need to check the layout item control because it's checked in `GridLayoutItem::new`

            inner.children.push(i);
            inner.base
        };
        

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Remove the children control in the layout. See also `remove_child_by_pos`.
        Note that the child control won't be hidden after being removed from the control.

        This method won't do anything if there is no control at the specified position.

        Panic:
        - If the layout is not initialized
    */
    pub fn remove_child<W: Into<ControlHandle>>(&self, c: W) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("GridLayout is not initialized");
            }

            let handle = c.into().hwnd().expect("Control must be window-like (HWND handle)");
            let index = inner.children.iter().position(|item| item.control == handle);
            match index {
                Some(i) => { inner.children.remove(i); },
                None => { return; }
            }
            
            inner.base
        };
        

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Remove the children control in the layout. See also `remove_child_by_pos`.
        Note that the child control won't be hidden after being removed from the control.

        This method won't do anything if there is no control at the specified position.

        Panic:
        - If the layout is not initialized
    */
    pub fn remove_child_by_pos(&self, col: u32, row: u32) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("GridLayout is not initialized");
            }

            let index = inner.children.iter().position(|item| item.col == col && item.row == row);
            match index {
                Some(i) => { inner.children.remove(i); },
                None => {}
            }
            
            inner.base
        };
        

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }


    /**
        Move the selected control to a new position in the grid layout. The old position
        becomes empty (as if `remove_child` was called). However it won't remove the control
        at the new position if there is one. 

        This method won't do anything if there is no control at the specified position.

        Panic:
        - If the layout is not initialized
    */
    pub fn move_child<W: Into<ControlHandle>>(&self, c: W, col: u32, row: u32) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("GridLayout is not initialized");
            }

            let handle = c.into().hwnd().expect("Control must be window-like (HWND handle)");
            let index = inner.children.iter().position(|item| item.control == handle);
            match index {
                Some(i) => { 
                    let mut child = inner.children.remove(i);
                    child.col = col;
                    child.row = row;
                    inner.children.push(child);
                }
                None => { return; }
            }
            
            inner.base
        };
        

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Move the selected control to a new position in the grid layout. The old position
        becomes empty (as if `remove_child` was called). However it won't remove the control
        at the new position if there is one. 

        This method won't do anything if there is no control at the specified position.

        Panic:
        - If the layout is not initialized
    */
    pub fn move_child_by_pos<W: Into<ControlHandle>>(&self, col: u32, row: u32, new_col: u32, new_row: u32) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("GridLayout is not initialized");
            }

            let index = inner.children.iter().position(|item| item.col == col && item.row == row);
            match index {
                Some(i) => { 
                    let mut child = inner.children.remove(i); 
                    child.col = new_col;
                    child.row = new_row;
                    inner.children.push(child);
                },
                None => {}
            }
            
            inner.base
        };
        

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Check if a window control is a children of the layout

        Panic:
        - If the layout is not initialized
        - If the child is not a window-like control
    */
    pub fn has_child<W: Into<ControlHandle>>(&self, c: W) -> bool {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("GridLayout is not initialized");
        }

        let handle = c.into().hwnd().expect("Children is not a window-like control (HWND handle)");
        inner.children.iter().any(|c| c.control == handle )
    }

    /// Resize the layout as if the parent window had the specified size.
    ///
    /// Arguments:
    ///   w: New width of the layout
    ///   h: New height of the layout
    ///
    ///  Panic:
    ///   - The layout must have been successfully built otherwise this function will panic.
    pub fn resize(&self, w: u32, h: u32) {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Grid layout is not bound to a parent control.")
        }
        self.update_layout(w, h);
    }

    /// Resize the layout to fit the parent window size
    ///
    /// Panic:
    ///   - The layout must have been successfully built otherwise this function will panic.
    pub fn fit(&self) {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Grid layout is not bound to a parent control.")
        }

        let (w, h) = unsafe { wh::get_window_size(inner.base) };
        self.update_layout(w, h);
    }

    /// Set the margins of the layout. The four values are in this order: top, right, bottom, left.
    pub fn margin(&self, m: [u32; 4]) {
        let mut inner = self.inner.borrow_mut();
        inner.margins = m;
    }

    /// Set the size of the space between the children in the layout. Default value is 5.
    pub fn spacing(&self, sp: u32) {
        let mut inner = self.inner.borrow_mut();
        inner.spacing = sp;
    }

    /// Sets the minimum size of the layout
    pub fn min_size(&self, sz: [u32; 2]) {
        let mut inner = self.inner.borrow_mut();
        inner.min_size = sz;
    }

    /// Sets the maximum size of the layout
    pub fn max_size(&self, sz: [u32; 2]) {
        let mut inner = self.inner.borrow_mut();
        inner.max_size = sz;
    }

    /// Set the number of column in the layout
    pub fn max_column(&self, count: Option<u32>) {
        let mut inner = self.inner.borrow_mut();
        inner.column_count = count;
    }

    /// Set the number of row in the layout
    pub fn max_row(&self, count: Option<u32>) {
        let mut inner = self.inner.borrow_mut();
        inner.row_count = count;
    }

    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let inner = self.inner.borrow();
        if inner.base.is_null() || inner.children.len() == 0 {
            return;
        }

        let [m_top, m_right, m_bottom, m_left] = inner.margins;
        let sp = inner.spacing;

        let children = &inner.children;

        let [min_w, min_h] = inner.min_size;
        if width < min_w { width = min_w; }
        if height < min_h { height = min_h; }

        let [max_w, max_h] = inner.max_size;
        if width > max_w { width = max_w; }
        if height > max_h { height = max_h; }

        let column_count = match inner.column_count {
            Some(c) => c,
            None => children.iter().map(|item| item.col + item.col_span).max().unwrap_or(1)
        };

        let row_count = match inner.row_count {
            Some(c) => c,
            None => children.iter().map(|item| item.row + item.row_span).max().unwrap_or(1)
        };

        if width < (m_right + m_left) + ((sp * 2) * column_count) {
            return;
        }

        if height < (m_top + m_bottom) + ((sp * 2) * row_count) {
            return;
        }

        // Apply margins
        width = width - m_right - m_left;
        height = height - m_top - m_bottom;

        // Apply spacing
        width = width - ((sp * 2) * column_count);
        height = height - ((sp * 2) * row_count);

        let item_width = width / column_count;
        let item_height = height / row_count;
        let sp2 = sp * 2;

        let mut columns = vec![item_width; column_count as usize];
        let mut rows = vec![item_height; row_count as usize];
        let extra_width = width - item_width * column_count;
        if extra_width > 0 {
            for x in &mut columns[0..(extra_width as usize)] {
                *x += 1;
            }
        }
        let extra_height = height - item_height * row_count;
        if extra_height > 0 {
            for x in &mut rows[0..(extra_height as usize)] {
                *x += 1;
            }
        }

        let mut last_handle = None;
        for item in inner.children.iter() {
            let x: u32 = m_left + (sp + (sp2 * item.col)) + columns[0..(item.col as usize)].iter().sum::<u32>();
            let y: u32 = m_top + (sp + (sp2 * item.row)) + rows[0..(item.row as usize)].iter().sum::<u32>();

            let local_width: u32 = &columns[(item.col as usize)..((item.col + item.col_span) as usize)].iter().sum::<u32>() + (sp2 * (item.col_span - 1));
            let local_height: u32 = &rows[(item.row as usize)..((item.row + item.row_span) as usize)].iter().sum::<u32>() + (sp2 * (item.row_span - 1));

            unsafe {
                wh::set_window_position(item.control, x as i32, y as i32);
                wh::set_window_size(item.control, local_width, local_height, false);
                wh::set_window_after(item.control, last_handle)
            }

            last_handle = Some(item.control);
        }
    }
}

impl Default for GridLayout {

    fn default() -> GridLayout {
        let inner = GridLayoutInner {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [5, 5, 5, 5],
            min_size: [0, 0],
            max_size: [u32::max_value(), u32::max_value()],
            column_count: None,
            row_count: None,
            spacing: 5,
        };

        GridLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}


/// Builder for a `GridLayout` struct
pub struct GridLayoutBuilder {
    layout: GridLayoutInner
}

impl GridLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> GridLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    /// Add a children to the layout at the position `col` and `row`.
    /// This is a shortcut over `child_item` for item with default span.
    /// The handle must be a window object otherwise the function will panic
    pub fn child<W: Into<ControlHandle>>(mut self, col: u32, row: u32, c: W) -> GridLayoutBuilder {
        let h = c.into().hwnd().expect("Child must be HWND");
        self.layout.children.push(GridLayoutItem {
            control: h,
            col,
            row,
            col_span: 1,
            row_span: 1,
        });

        self
    }

    /// Add a children to the layout
    /// The handle must be a window object otherwise the function will panic
    pub fn child_item(mut self, item: GridLayoutItem) -> GridLayoutBuilder {
        self.layout.children.push(item);
        self
    }

    /// Set the margins of the layout. The four values are in this order: top, right, bottom, left.
    pub fn margin(mut self, m: [u32; 4]) -> GridLayoutBuilder {
        self.layout.margins = m;
        self
    }

    /// Set the size of the space between the children in the layout. Default value is 5.
    pub fn spacing(mut self, sp: u32) -> GridLayoutBuilder {
        self.layout.spacing = sp;
        self
    }

    /// Sets the minimum size of the layout
    pub fn min_size(mut self, sz: [u32; 2]) -> GridLayoutBuilder {
        self.layout.min_size = sz;
        self
    }

    /// Sets the maximum size of the layout
    pub fn max_size(mut self, sz: [u32; 2]) -> GridLayoutBuilder {
        self.layout.max_size = sz;
        self
    }

    /// Set the number of column in the layout
    pub fn max_column(mut self, count: Option<u32>) -> GridLayoutBuilder {
        self.layout.column_count = count;
        self
    }

    /// Set the number of row in the layout
    pub fn max_row(mut self, count: Option<u32>) -> GridLayoutBuilder {
        self.layout.row_count = count;
        self
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(self, layout: &GridLayout) -> Result<(), NwgError> {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        if self.layout.base.is_null() {
            return Err(NwgError::layout_create("Gridlayout does not have a parent."));
        }

        // Checks if the layouts cell or row are outside max_column or max_row
        if let Some(max_row) = self.layout.row_count {
            if let Some(item) = self.layout.children.iter().find(|c| c.row >= max_row) {
                return Err(NwgError::layout_create(format!("A layout item row is bigger or equal than the max number of row. {} >= {}", item.row, max_row)));
            }
        }

        if let Some(max_column) = self.layout.column_count {
            if let Some(item) = self.layout.children.iter().find(|c| c.col >= max_column) {
                return Err(NwgError::layout_create(format!("A layout item column is bigger or equal than the max number of column. {} >= {}", item.col, max_column)));
            }
        }
        
        let (w, h) = unsafe { wh::get_window_size(self.layout.base) };
        let base_handle = ControlHandle::Hwnd(self.layout.base);

        // Saves the new layout. TODO: should free the old one too (if any)
        {
            let mut layout_inner = layout.inner.borrow_mut();
            *layout_inner = self.layout;        
        }

        // Initial layout update
        layout.update_layout(w, h);
       
        // Bind the event handler
        let event_layout = layout.clone();
        let cb = move |_h, msg, _w, l| {
            if msg == WM_SIZE {
                let size = l as u32;
                let width = LOWORD(size) as i32;
                let height = HIWORD(size) as i32;
                let (w, h) = unsafe { crate::win32::high_dpi::physical_to_logical(width, height) };
                GridLayout::update_layout(&event_layout, w as u32, h as u32);
            }
            None
        };

        /// Keep generating ids so that multiple layouts can be applied to the same parent
        use std::sync::atomic::{AtomicUsize, Ordering};
        static BOX_LAYOUT_ID: AtomicUsize = AtomicUsize::new(0x8FFF); 
        bind_raw_event_handler_inner(&base_handle, BOX_LAYOUT_ID.fetch_add(1, Ordering::SeqCst), cb).unwrap();

        Ok(())
    }

}
