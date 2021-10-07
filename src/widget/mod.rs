mod common;
mod dimensions;
mod layout_hacks;
mod menu;
mod polymorphic;

use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use chiropterm::*;

pub use self::common::WidgetCommon;
pub use self::dimensions::{InternalWidgetDimensions, WidgetDimensions};
pub use self::layout_hacks::LayoutHacks;
pub use self::menu::WidgetMenu;
pub use self::polymorphic::AnyWidget;

use super::UI;

pub struct Widget<T: Widgetlike> {
    // TODO: Instead use a ref inside an arena allocator (not bump, we need drop)
    state: Rc<RefCell<WidgetCommon<T>>>,
}

impl<T: Widgetlike> Widget<T> {
    pub fn new() -> Self {
        Widget { 
            state: Rc::new(RefCell::new(WidgetCommon::new(T::create()))),
        }
    }

    pub fn share(&self) -> Self {
        Widget { 
            state: self.state.clone(), 
        }
    }

    pub fn borrow(&self) -> Ref<WidgetCommon<T>> {
        self.state.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<WidgetCommon<T>> {
        self.state.borrow_mut()
    }

    pub fn setup(&self, f: impl FnOnce(&mut T)) -> Self {
        f(&mut self.state.borrow_mut().unique);
        self.share()
    }

    pub fn draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame>) {
        let brush = self.internal_estimate_dimensions(&ui, brush.rect().width()).tailor(brush);
        let offset = brush.cursor_offset();
        let widget_menu = WidgetMenu { 
            ui, state: self.state.clone(), menu, brush_offset: offset,
        };
        if brush.clip().is_empty() {
            self.state.borrow().skip_draw(brush, widget_menu)
        } else {
            self.state.borrow().draw(brush, widget_menu);
        }
    }

    pub fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        let mut dims = self.internal_estimate_dimensions(ui, width).to_external();
        dims = self.state.borrow().apply_layout_hacks(dims);
        dims
    }

    fn internal_estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        self.clear_layout_cache_if_needed(ui);
        self.state.borrow().estimate_dimensions(ui, width)
    }

    pub(in super) fn clear_layout_cache_if_needed(&self, ui: &UI) {
        self.state.borrow_mut().clear_layout_cache_if_needed(ui)
    }
}

pub trait Widgetlike: 'static+Sized {
    fn create() -> Self;

    fn skip_draw<'frame>(&self, _selected: bool, _brush: Brush, _menu: WidgetMenu<'frame, Self>) {
        // NOTE: You can implement custom behavior if your widget must do work to pretend it was drawn when it wasn't drawn 
        // (ex: reshape to match the brush)
    }
    fn draw<'frame>(&self, selected: bool, brush: Brush, menu: WidgetMenu<'frame, Self>);
    fn estimate_dimensions(&self, ui: &UI, width: isize) ->InternalWidgetDimensions;
    fn clear_layout_cache(&self, ui: &UI);
    fn layout_hacks(&self) -> LayoutHacks;
}