use core::cell::RefCell;

use agx_definitions::{
    Color, Drawable, LayerSlice, NestedLayerSlice, Point, Rect, RectInsets, Size,
};
use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::{boxed::Box, vec::Vec};

use crate::{bordered::Bordered, ui_elements::UIElement};
use axle_rt::printf;

pub struct View {
    container: RefCell<Option<RefCell<Weak<dyn NestedLayerSlice>>>>,
    frame: RefCell<Rect>,
    left_click_cb: RefCell<Option<Box<dyn Fn(&Self)>>>,
    background_color: Color,
    sizer: RefCell<Box<dyn Fn(&Self, Size) -> Rect>>,
    currently_contains_mouse_int: RefCell<bool>,

    current_inner_content_frame: RefCell<Rect>,

    sub_elements: RefCell<Vec<Rc<dyn UIElement>>>,
    sub_elements_containing_mouse: RefCell<Vec<Rc<dyn UIElement>>>,

    border_enabled: RefCell<bool>,
}

impl View {
    pub fn new<F: 'static + Fn(&Self, Size) -> Rect>(background_color: Color, sizer: F) -> Self {
        View {
            container: RefCell::new(None),
            frame: RefCell::new(Rect::zero()),
            current_inner_content_frame: RefCell::new(Rect::zero()),
            left_click_cb: RefCell::new(None),
            background_color,
            sizer: RefCell::new(Box::new(sizer)),
            currently_contains_mouse_int: RefCell::new(false),
            sub_elements: RefCell::new(Vec::new()),
            sub_elements_containing_mouse: RefCell::new(Vec::new()),
            border_enabled: RefCell::new(true),
        }
    }

    pub fn set_border_enabled(&self, enabled: bool) {
        self.border_enabled.replace(enabled);
    }

    pub fn on_left_click<F: 'static + Fn(&Self)>(&self, f: F) {
        *self.left_click_cb.borrow_mut() = Some(Box::new(f));
    }

    pub fn add_component(self: Rc<Self>, elem: Rc<dyn UIElement>) {
        // Ensure the component has a frame by running its sizer
        elem.handle_superview_resize(self.current_inner_content_frame.borrow().size);

        // Set up a link to the parent
        elem.set_parent(Rc::downgrade(&(Rc::clone(&self) as _)));

        self.sub_elements.borrow_mut().push(elem);
    }
}

impl NestedLayerSlice for View {
    fn get_parent(&self) -> Option<Weak<dyn NestedLayerSlice>> {
        Some(Weak::clone(
            &self.container.borrow().as_ref().unwrap().borrow(),
        ))
    }

    fn set_parent(&self, parent: Weak<dyn NestedLayerSlice>) {
        self.container.replace(Some(RefCell::new(parent)));
    }
}

impl Bordered for View {
    fn draw_inner_content(&self, _outer_frame: Rect, onto: &mut LayerSlice) {
        onto.fill(self.background_color);

        let sub_elements = &self.sub_elements.borrow();
        for elem in sub_elements.iter() {
            elem.draw();
        }
    }

    fn set_interior_content_frame(&self, inner_content_frame: Rect) {
        self.current_inner_content_frame
            .replace(inner_content_frame);
    }

    fn get_interior_content_frame(&self) -> Rect {
        *self.current_inner_content_frame.borrow()
    }

    fn border_enabled(&self) -> bool {
        *self.border_enabled.borrow()
    }

    fn border_insets(&self) -> RectInsets {
        match *self.border_enabled.borrow() {
            true => RectInsets::new(11, 11, 11, 11),
            false => RectInsets::zero(),
        }
    }
}

impl Drawable for View {
    fn frame(&self) -> Rect {
        *self.frame.borrow()
    }

    fn content_frame(&self) -> Rect {
        Bordered::content_frame(self)
    }

    fn draw(&self) {
        //panic!("View is drawn via Bordered::draw()");
        Bordered::draw(self);
    }
}

impl UIElement for View {
    fn handle_left_click(&self) {
        let maybe_cb = &*self.left_click_cb.borrow();
        if let Some(cb) = maybe_cb {
            (cb)(self);
        }

        let elems_containing_mouse = &mut *self.sub_elements_containing_mouse.borrow_mut();
        for elem in elems_containing_mouse {
            elem.handle_left_click();
        }
    }

    fn handle_superview_resize(&self, superview_size: Size) {
        let sizer = &*self.sizer.borrow();
        let frame = sizer(self, superview_size);
        self.frame.replace(frame);

        let inner_content_frame = frame.apply_insets(self.border_insets());

        let elems = &*self.sub_elements.borrow();
        for elem in elems {
            elem.handle_superview_resize(inner_content_frame.size);
        }
    }

    fn handle_mouse_entered(&self) {
        *self.currently_contains_mouse_int.borrow_mut() = true;
        self.draw_border();
    }

    fn handle_mouse_exited(&self) {
        *self.currently_contains_mouse_int.borrow_mut() = false;

        let inner_content_origin = (*self.current_inner_content_frame.borrow()).origin;

        let elems_containing_mouse = &mut *self.sub_elements_containing_mouse.borrow_mut();
        for elem in elems_containing_mouse.drain(..) {
            elem.handle_mouse_exited();
        }

        self.draw_border();
    }

    fn handle_mouse_moved(&self, mouse_point: Point) {
        let elems = &*self.sub_elements.borrow();
        let elems_containing_mouse = &mut *self.sub_elements_containing_mouse.borrow_mut();

        let inner_content_origin = (*self.current_inner_content_frame.borrow()).origin;
        let mouse_to_inner_coordinate_system = mouse_point - inner_content_origin;

        for elem in elems {
            let elem_contains_mouse = elem.frame().contains(mouse_to_inner_coordinate_system);

            // Did this element previously bound the mouse?
            if let Some(index) = elems_containing_mouse
                .iter()
                .position(|e| Rc::ptr_eq(e, elem))
            {
                // Did the mouse just exit this element?
                if !elem_contains_mouse {
                    elem.handle_mouse_exited();
                    // We don't need to preserve ordering, so swap_remove is OK
                    elems_containing_mouse.swap_remove(index);
                }
            } else if elem_contains_mouse {
                elem.handle_mouse_entered();
                elems_containing_mouse.push(Rc::clone(elem));
            }
        }

        for elem in elems_containing_mouse {
            let elem_local_point = mouse_point - elem.frame().origin;
            elem.handle_mouse_moved(elem_local_point);
        }
    }

    fn currently_contains_mouse(&self) -> bool {
        *self.currently_contains_mouse_int.borrow()
    }
}