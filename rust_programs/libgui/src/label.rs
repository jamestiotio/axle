use core::cell::RefCell;

use agx_definitions::{Color, Drawable, NestedLayerSlice, Point, Rect, Size};
use alloc::boxed::Box;
use alloc::{
    rc::{Rc, Weak},
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::{font::draw_char, ui_elements::UIElement};

pub struct Label {
    sizer: RefCell<Box<dyn Fn(&Self, Size) -> Rect>>,
    // TODO(PT): Remove the nested RefCell?
    container: RefCell<Option<RefCell<Weak<dyn NestedLayerSlice>>>>,
    frame: RefCell<Rect>,
    pub text: RefCell<String>,
    color: Color,
}

impl Label {
    pub fn new<F: 'static + Fn(&Label, Size) -> Rect>(text: &str, color: Color, sizer: F) -> Self {
        Self {
            sizer: RefCell::new(Box::new(sizer)),
            container: RefCell::new(None),
            frame: RefCell::new(Rect::zero()),
            text: RefCell::new(text.to_string()),
            color,
        }
    }

    pub fn set_text(&self, text: &str) {
        self.text.replace(text.to_string());
    }

    pub fn set_frame(self: &Rc<Self>, frame: Rect) {
        self.frame.replace(frame);
    }
}

impl UIElement for Label {
    fn handle_superview_resize(&self, superview_size: Size) {
        let sizer = &*self.sizer.borrow();
        let frame = sizer(self, superview_size);
        self.frame.replace(frame);
    }
}

impl NestedLayerSlice for Label {
    fn get_parent(&self) -> Option<Weak<dyn NestedLayerSlice>> {
        Some(Weak::clone(
            &self.container.borrow().as_ref().unwrap().borrow(),
        ))
    }

    fn set_parent(&self, parent: Weak<dyn NestedLayerSlice>) {
        self.container.replace(Some(RefCell::new(parent)));
    }

    fn get_slice_for_render(&self) -> Box<dyn agx_definitions::LikeLayerSlice> {
        self.get_slice()
    }
}

impl Drawable for Label {
    fn frame(&self) -> Rect {
        *self.frame.borrow()
    }

    fn content_frame(&self) -> Rect {
        Rect::from_parts(Point::zero(), self.frame().size)
    }

    fn draw(&self) -> Vec<Rect> {
        let onto = &mut self.get_slice();
        let font_size = Size::new(8, 10);
        let mut cursor = Point::zero();
        for ch in self.text.borrow().chars() {
            draw_char(onto, ch, &cursor, self.color, &font_size);
            cursor.x += font_size.width;
        }
        // TODO: Damages
        vec![]
    }
}
