use crate::controls::ControlHandle;
use crate::win32::window_helper as wh;
use crate::win32::window::{RawEventHandler, unbind_raw_event_handler, bind_raw_event_handler_inner};
use crate::NwgError;
use winapi::shared::windef::HWND;
use std::{ptr, rc::Rc, cell::{RefCell, RefMut, Ref} };

use stretch::{
    number::Number,
    geometry::{Point, Size, Rect},
    node::{Node, Stretch},
    style::*
};


#[derive(Debug)]
pub struct FlexboxLayoutItem {
    /// The handle to the control in the item
    control: HWND,
    style: Style,
}

pub enum FlexboxLayoutChild {
    Item(FlexboxLayoutItem),
    Flexbox(FlexboxLayout)
}

impl FlexboxLayoutChild {
    fn modify_style<F>(&mut self, fnc: F) where F: Fn(&mut Style) {
        match self {
            FlexboxLayoutChild::Item(item) => fnc(&mut item.style),
            FlexboxLayoutChild::Flexbox(layout) => fnc(&mut layout.inner.borrow_mut().style),
        }
    }
}

/// This is the inner data shared between the callback and the application
struct FlexboxLayoutInner {
    base: HWND,
    handler: Option<RawEventHandler>,
    style: Style,
    children: Vec<FlexboxLayoutChild>,
    parent_layout: Option<FlexboxLayout>,
}


/**
    A flexbox layout that organizes the children control in a parent control.
    Flexbox uses the stretch library internally ( https://github.com/vislyhq/stretch ).

    FlexboxLayout requires the `flexbox` feature.
*/
#[derive(Clone)]
pub struct FlexboxLayout {
    inner: Rc<RefCell<FlexboxLayoutInner>>
}

impl FlexboxLayout {

    pub fn builder() -> FlexboxLayoutBuilder {
        let layout = FlexboxLayoutInner {
            base: ptr::null_mut(),
            handler: None,
            style: Default::default(),
            children: Vec::new(),
            parent_layout: None,
        };

        FlexboxLayoutBuilder { layout, current_index: None, auto_size: true, auto_spacing: Some(5) }
    }

    /**
        Returns the style of the parent control
        
        Panic:
        - The layout must have been successfully built otherwise this function will panic.
    */
    pub fn style(&self) -> Style {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }

        inner.style.clone()
    }

    /**
        Sets the style of the layout parent control

        Panic:
        - The layout must have been successfully built otherwise this function will panic.
    */
    pub fn set_style(&self, style: Style) {
        let mut inner = self.inner.borrow_mut();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }

        inner.style = style;
    }

    /**
        Add a new children in the layout with the stretch style. 
        
        Panic:
        * If the control is not a window-like control
        * If the layout was not initialized
    */
    pub fn add_child<W: Into<ControlHandle>>(&self, c: W, style: Style) -> Result<(), stretch::Error> {
        {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("Flexbox layout is not yet initialized!");
            }
    
            let item = FlexboxLayoutItem {
                control: c.into().hwnd().expect("Control must be window like (HWND handle)"),
                style
            };
    
            inner.children.push(FlexboxLayoutChild::Item(item));
        }

        self.fit()
    }

    /**
        Remove a children from the layout
        
        Panic:
        * If the control is not a window-like control
        * If the control is not in the layout (see `has_child`)
        * If the layout was not initialized
    */
    pub fn remove_child<W: Into<ControlHandle>>(&self, c: W) {
        let mut inner = self.inner.borrow_mut();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }

        let handle = c.into().hwnd().expect("Control must be window like (HWND handle)");
        let index = inner.children.iter()
            .position(|child| child.is_item() && child.as_item().control == handle);

        match index {
            Some(i) => { inner.children.remove(i); },
            None => { panic!("Control was not found in layout"); }
        }
    }

    /**
        Check if the selected control is a children in the layout.
        Does not check in the sublayouts

        Panic:
        * If the control is not a window-like control.
        * If the layout was not initialized
    */
    pub fn has_child<W: Into<ControlHandle>>(&self, c: W) -> bool {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }

        let handle = c.into().hwnd().expect("Control must be window like (HWND handle)");
        inner.children.iter().any(|child| child.is_item() && child.as_item().control == handle)
    }

    /**
        Borrow the inner value of the flexbox layout. While the returned value lives, calling other method
        of the the flexbox layout that modify the inner state will cause a panic. Simple looktup (ex: `has_child`) will still work.

        Panic:
        - The layout must have been successfully built otherwise this function will panic.
    */
    pub fn children(&self) -> FlexboxLayoutChildren {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }
        
        FlexboxLayoutChildren {
            inner
        }
    }

    /**
        Borrow the inner value of the flexbox layout as mutable. While the returned value lives, calling other method
        of the the flexbox layout will cause a panic.

        If the children of the layout were modified, call `fit` to update the layout after `FlexboxLayoutChildrenMut` is dropped.

        Panic:
        - The layout must have been successfully built otherwise this function will panic.
    */
    pub fn children_mut(&self) -> FlexboxLayoutChildrenMut {
        let inner = self.inner.borrow_mut();
        if inner.base.is_null() {
            panic!("Flexbox layout is not yet initialized!");
        }

        FlexboxLayoutChildrenMut {
            inner
        }
    }

    /** 
        Resize the layout to fit the parent window size
        
        Panic:
        - The layout must have been successfully built otherwise this function will panic.
    */
    pub fn fit(&self) -> Result<(), stretch::Error> {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("FlexboxLayout is not bound to a parent control.")
        }

        if let Some(parent_layout) = &inner.parent_layout {
            parent_layout.fit()
        } 
        else {
            let (w, h) = unsafe { wh::get_window_size(inner.base) };
            self.update_layout(w, h, (0, 0))
        }
    }

    // Utility function to compile tree of children nodes for layout purposes
    // Also returns the total number of children items to allow cleaner deferred positioning
    fn build_child_nodes(children: &Vec<FlexboxLayoutChild>, stretch: &mut Stretch) -> Result<(usize, Vec<Node>), stretch::Error> {
        let mut nodes = Vec::new();
        let mut item_count = 0;

        for child in children.iter() {
            match child {
                FlexboxLayoutChild::Item(child) =>{
                    nodes.push(stretch.new_node(child.style, Vec::new())?);
                    item_count += 1;
                },
                FlexboxLayoutChild::Flexbox(child) => {
                    let (child_count, child_nodes) = FlexboxLayout::build_child_nodes(child.children().children(), stretch)?;
                    nodes.push(stretch.new_node(child.style(), child_nodes)?);
                    item_count += child_count;
                },
            };
        }

        Ok((item_count, nodes))
    }

    // Applies the calculated item positions for this layout
    // Uses deferred window positioning to prevent rendering artefacts
    fn apply_layout_deferred(positioner: &mut wh::DeferredWindowPositioner, stretch: &mut Stretch, nodes: Vec<Node>, children: &Vec<FlexboxLayoutChild>, last_handle: &mut Option<HWND>, offset: (i32, i32)) -> Result<(), stretch::Error> {
        use FlexboxLayoutChild as Child;

        for (node, child) in nodes.into_iter().zip(children.iter()) {
            let layout = stretch.layout(node)?;
            let Point { x, y } = layout.location;
            let Size { width, height } = layout.size;

            match child {
                Child::Item(child) => {
                    positioner.defer_pos(child.control, last_handle.unwrap_or(std::ptr::null_mut()), x as i32 + offset.0, y as i32 + offset.1, width as i32, height as i32).ok();
                    last_handle.replace(child.control);                    
                },
                Child::Flexbox(child) => {
                    let children_nodes = stretch.children(node)?;
                    FlexboxLayout::apply_layout_deferred(positioner, stretch, children_nodes, child.children().children(), last_handle, (x as i32, y as i32))?;
                }
            }
            
        }

        Ok(())
    }

    // Applies the calculated item positions for this layout
    // Uses immediate window positioning, which might cause visual artefacts in some cases
    fn apply_layout_immediate(stretch: &mut Stretch, nodes: Vec<Node>, children: &Vec<FlexboxLayoutChild>, last_handle: &mut Option<HWND>, offset: (i32, i32)) -> Result<(), stretch::Error> {
        use FlexboxLayoutChild as Child;

        for (node, child) in nodes.into_iter().zip(children.iter()) {
            let layout = stretch.layout(node)?;
            let Point { x, y } = layout.location;
            let Size { width, height } = layout.size;

            match child {
                Child::Item(child) => unsafe {
                    wh::set_window_position(child.control, x as i32 + offset.0, y as i32 + offset.1);
                    wh::set_window_size(child.control, width as u32, height as u32, false);
                    wh::set_window_after(child.control, *last_handle);
                    last_handle.replace(child.control);                    
                },
                Child::Flexbox(child) => {
                    let children_nodes = stretch.children(node)?;
                    FlexboxLayout::apply_layout_immediate(stretch, children_nodes, child.children().children(), last_handle, (x as i32, y as i32))?;
                }
            }
            
        }

        Ok(())
    }

    fn update_layout(&self, width: u32, height: u32, offset: (i32, i32)) -> Result<(), stretch::Error> {
        let inner = self.inner.borrow();
        if inner.base.is_null() || inner.children.len() == 0 {
            return Ok(());
        }

        let mut stretch = Stretch::new();
        let (item_count, nodes) = FlexboxLayout::build_child_nodes(&inner.children, &mut stretch)?;

        let mut style = inner.style.clone();
        style.size = Size { width: Dimension::Points(width as f32), height: Dimension::Points(height as f32) };
        let node = stretch.new_node(style, nodes.clone())?;

        stretch.compute_layout(node, Size::undefined())?;

        // Keep a fallback case to prevent panics if the layout is too large to be deferred
        if let Ok(mut positioner) = wh::DeferredWindowPositioner::new(item_count as i32) {
            let layout_result = FlexboxLayout::apply_layout_deferred(&mut positioner, &mut stretch, nodes, self.children().children(), &mut None, offset);
            positioner.end();
    
            layout_result
        }
        else {
            FlexboxLayout::apply_layout_immediate(&mut stretch, nodes, self.children().children(), &mut None, offset)
        }
    }
}

pub struct FlexboxLayoutBuilder {
    layout: FlexboxLayoutInner,
    current_index: Option<usize>,
    auto_size: bool,
    auto_spacing: Option<u32>
}

impl FlexboxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> FlexboxLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    /// Add a new child to the layout build.
    /// Panics if `child` is not a window-like control.
    pub fn child<W: Into<ControlHandle>>(mut self, child: W) -> FlexboxLayoutBuilder {
        self.current_index = Some(self.layout.children.len());
        
        let item = FlexboxLayoutItem {
            control: child.into().hwnd().unwrap(),
            style: Style::default()
        };

        self.layout.children.push(FlexboxLayoutChild::Item(item));

        self
    }

    /// Add a new child layout to the layout build.
    pub fn child_layout(mut self, child: &FlexboxLayout) -> FlexboxLayoutBuilder {
        self.current_index = Some(self.layout.children.len());
        
        self.layout.children.push(FlexboxLayoutChild::Flexbox(child.clone()));

        self
    }

    /// Make it so that the children of the layout all have equal size
    /// This flags is erased when `size`, `max_size`, or `min_size` is set on the children.
    pub fn auto_size(mut self, auto: bool) -> FlexboxLayoutBuilder {
        self.auto_size = auto;
        self
    }

    /// Automatically generate padding and margin for the parent layout and the children from the selected value.
    /// This flags is erased when `padding` is called on the layout or when `child_margin` is called on the children
    pub fn auto_spacing(mut self, auto: Option<u32>) -> FlexboxLayoutBuilder {
        self.auto_spacing = auto;
        self
    }

    //
    // Base layout style
    //

    pub fn direction(mut self, value: Direction) -> FlexboxLayoutBuilder {
        self.layout.style.direction = value;
        self
    }

    pub fn flex_direction(mut self, value: FlexDirection) -> FlexboxLayoutBuilder {
        self.layout.style.flex_direction = value;
        self
    }

    pub fn flex_wrap(mut self, value: FlexWrap) -> FlexboxLayoutBuilder {
        self.layout.style.flex_wrap = value;
        self
    }

    pub fn overflow(mut self, value: Overflow) -> FlexboxLayoutBuilder {
        self.layout.style.overflow = value;
        self
    }

    pub fn align_items(mut self, value: AlignItems) -> FlexboxLayoutBuilder {
        self.layout.style.align_items = value;
        self
    }

    pub fn align_content(mut self, value: AlignContent) -> FlexboxLayoutBuilder {
        self.layout.style.align_content = value;
        self
    }

    pub fn justify_content(mut self, value: JustifyContent) -> FlexboxLayoutBuilder {
        self.layout.style.justify_content = value;
        self
    }

    pub fn padding(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.padding = value;
        self.auto_spacing = None;
        self
    }

    pub fn border(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.border = value;
        self
    }

    pub fn min_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.min_size = value;
        self
    }

    pub fn max_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.max_size = value;
        self
    }

    pub fn aspect_ratio(mut self, value: Number) -> FlexboxLayoutBuilder {
        self.layout.style.aspect_ratio = value;
        self
    }

    //
    // Child layout style
    //

    /// Set the size of of the current child.
    /// Panics if `child` was not called before.
    pub fn child_size(mut self, size: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.size = size);
        self.auto_size = false;
        self
    }

    /// Set the position of the current child.
    /// Panics if `child` was not called before.
    pub fn child_position(mut self, position: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.position = position);
        self
    }

    /// Set the margin of the current child.
    /// Panics if `child` was not called before.
    pub fn child_margin(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.margin = value);
        self.auto_spacing = None;
        self
    }

    /// Set the min size of the current child.
    /// Panics if `child` was not called before.
    pub fn child_min_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.min_size = value);
        self.auto_size = false;
        self
    }

    /// Set the max size of the current child.
    /// Panics if `child` was not called before.
    pub fn child_max_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.max_size = value);
        self.auto_size = false;
        self
    }

    /// Panics if `child` was not called before.
    pub fn child_flex_grow(mut self, value: f32) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.flex_grow = value);
        self.auto_size = false;
        self
    }

    /// Panics if `child` was not called before.
    pub fn child_flex_shrink(mut self, value: f32) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.flex_shrink = value);
        self.auto_size = false;
        self
    }

    /// Panics if `child` was not called before.
    pub fn child_flex_basis(mut self, value: Dimension) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.flex_basis = value);
        self.auto_size = false;
        self
    }

    /// Panics if `child` was not called before.
    pub fn child_align_self(mut self, value: AlignSelf) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| s.align_self = value);
        self
    }


    /**
        Directly set the style parameter of the current child. Panics if `child` was not called before.
        
        If defining style is too verbose, other method such as `size` can be used.
    */
    pub fn style(mut self, style: Style) -> FlexboxLayoutBuilder {
        self.modify_current_child_style(|s| *s = style);
        self
    }

    fn modify_current_child_style<F>(&mut self, fnc: F) where F: Fn(&mut Style) {
        assert!(self.current_index.is_some(), "No current children");

        let index = self.current_index.unwrap();

        self.layout.children[index].modify_style(|s| fnc(s));
    }

    /// Build the layout object and bind the callback.
    pub fn build(mut self, layout: &FlexboxLayout) -> Result<(), NwgError> {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        if self.layout.base.is_null() {
            return Err(NwgError::layout_create("Flexboxlayout does not have a parent."));
        }

        let (w, h) = unsafe { wh::get_window_size(self.layout.base) };
        let base_handle = ControlHandle::Hwnd(self.layout.base);

        // Auto compute size if enabled
        if self.auto_size {
            let children_count = self.layout.children.len();
            let size = 1.0f32 / (children_count as f32);
            for child in self.layout.children.iter_mut() {
                let child_size = match &self.layout.style.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        Size { width: Dimension::Percent(size), height: Dimension::Auto }
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        Size { width: Dimension::Auto, height: Dimension::Percent(size) }
                    }
                };

                child.modify_style(|s| s.size = child_size);
            }
        }

        // Auto spacing if enabled
        if let Some(spacing) = self.auto_spacing {
            let spacing = Dimension::Points(spacing as f32);
            let spacing = Rect { start: spacing, end: spacing, top: spacing, bottom: spacing};
            self.layout.style.padding = spacing;
            for child in self.layout.children.iter_mut() {
                child.modify_style(|s| s.margin = spacing);
            }
        }

        // Saves the new layout. Free the old layout (if there is one)
        {
            let mut layout_inner = layout.inner.borrow_mut();
            if layout_inner.handler.is_some() {
                drop(unbind_raw_event_handler(layout_inner.handler.as_ref().unwrap()));
            }
            
            *layout_inner = self.layout;        
        }

        // Sets the parent_layout of any child layout to this layout
        for child in layout.inner.borrow_mut().children.iter_mut() {
            match child {
                FlexboxLayoutChild::Item(_) => {},
                FlexboxLayoutChild::Flexbox(child_layout) => { 
                    child_layout.inner.borrow_mut().parent_layout.replace(layout.clone()); 
                },
            }
        }

        // Initial layout update
        layout.update_layout(w, h, (0, 0)).expect("Failed to compute layout");

        // Fetch a new ID for the layout handler
        use std::sync::atomic::{AtomicUsize, Ordering};
        static FLEX_LAYOUT_ID: AtomicUsize = AtomicUsize::new(0x9FFF); 
        let handler_id = FLEX_LAYOUT_ID.fetch_add(1, Ordering::SeqCst);
 
        // Bind the event handler
        let event_layout = layout.clone();
        let cb = move |_h, msg, _w, l| {
            if msg == WM_SIZE {
                let size = l as u32;
                let width = LOWORD(size) as i32;
                let height = HIWORD(size) as i32;
                let (w, h) = unsafe { crate::win32::high_dpi::physical_to_logical(width, height) };
                FlexboxLayout::update_layout(&event_layout, w as u32, h as u32, (0, 0)).expect("Failed to compute layout!");
            }
            None
        };

        {
            let mut layout_inner = layout.inner.borrow_mut();
            layout_inner.handler = Some(bind_raw_event_handler_inner(&base_handle, handler_id, cb).unwrap());
        }

        Ok(())
    }

    /// Build a "partial" layout object - this layout has no direct callback and needs to be added to a parent layout using child_layout
    pub fn build_partial(mut self, layout: &FlexboxLayout) -> Result<(), NwgError> {
        if self.layout.base.is_null() {
            return Err(NwgError::layout_create("Flexboxlayout does not have a parent."));
        }

        // Auto compute size if enabled
        if self.auto_size {
            let children_count = self.layout.children.len();
            let size = 1.0f32 / (children_count as f32);
            for child in self.layout.children.iter_mut() {               
                let child_size = match &self.layout.style.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        Size { width: Dimension::Percent(size), height: Dimension::Auto }
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        Size { width: Dimension::Auto, height: Dimension::Percent(size) }
                    }
                };

                child.modify_style(|s| s.size = child_size);
            }
        }

        // Auto spacing if enabled
        if let Some(spacing) = self.auto_spacing {
            let spacing = Dimension::Points(spacing as f32);
            let spacing = Rect { start: spacing, end: spacing, top: spacing, bottom: spacing};
            self.layout.style.padding = spacing;
            for child in self.layout.children.iter_mut() {
                child.modify_style(|s| s.margin = spacing);
            }
        }

        // Saves the new layout. Free the old layout (if there is one)
        {
            let mut layout_inner = layout.inner.borrow_mut();
            if layout_inner.handler.is_some() {
                drop(unbind_raw_event_handler(layout_inner.handler.as_ref().unwrap()));
            }
            
            *layout_inner = self.layout;        
        }

        Ok(())
    }
}

impl Default for FlexboxLayout {

    fn default() -> FlexboxLayout {
        let inner = FlexboxLayoutInner {
            base: ptr::null_mut(),
            handler: None,
            children: Vec::new(),
            style: Default::default(),
            parent_layout: None,
        };

        FlexboxLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}


impl FlexboxLayoutChild {

    pub fn is_item(&self) -> bool {
        match self {
            FlexboxLayoutChild::Item(_) => true,
            _ => false
        }
    }

    pub fn as_item<'a>(&'a self) -> &'a FlexboxLayoutItem {
        match self {
            FlexboxLayoutChild::Item(i) => i,
            _ => panic!("FlexboxLayoutChild is not an item")
        }
    }

    pub fn as_item_mut<'a>(&'a mut self) -> &'a mut FlexboxLayoutItem {
        match self {
            FlexboxLayoutChild::Item(i) => i,
            _ => panic!("FlexboxLayoutChild is not an item")
        }
    }

    pub fn is_flexbox(&self) -> bool {
        match self {
            FlexboxLayoutChild::Flexbox(_) => true,
            _ => false
        }
    }

}


/**
    A wrapper that expose the inner collection of a flexboxlayout.
*/
pub struct FlexboxLayoutChildrenMut<'a> {
    inner: RefMut<'a, FlexboxLayoutInner>
}

impl<'a> FlexboxLayoutChildrenMut<'a> {
    pub fn children<'b>(&'b mut self) -> &'b mut Vec<FlexboxLayoutChild> {
        &mut self.inner.children
    }
}


pub struct FlexboxLayoutChildren<'a> {
    inner: Ref<'a, FlexboxLayoutInner>
}

impl<'a> FlexboxLayoutChildren<'a> {
    pub fn children<'b>(&'b self) -> &'b Vec<FlexboxLayoutChild> {
        &self.inner.children
    }
}