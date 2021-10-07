use std::cell::Cell;

use chiropterm::{Brush};

use crate::{UI, ui::Selection};

use super::{WidgetDimensions, InternalWidgetDimensions, WidgetMenu, Widgetlike};

pub struct WidgetCommon<T> {
    pub unique: T,
    pub(in crate) selection: Selection,
    pub(in crate) layout_token: Cell<u64>,

    last_dimensions: Cell<(isize, InternalWidgetDimensions)>,
}

// TODO: Store last 3 dimension estimates, since some widgets (like scrollable) try several widths
impl<T: Widgetlike> WidgetCommon<T> {
    pub fn new(value: T) -> Self {
        WidgetCommon {
            unique: value,
            selection: Selection::not_selected(),
            last_dimensions: Cell::new((-1, InternalWidgetDimensions::zero())),
            layout_token: Cell::new(0),
        }
    }

    pub fn skip_draw<'frame>(&self, brush: Brush, menu: WidgetMenu<'frame, T>) {
        self.unique.skip_draw(menu.ui.is_selected(self.selection), brush, menu)
    }

    pub fn draw<'frame>(&self, brush: Brush, menu: WidgetMenu<'frame, T>) {
        self.unique.draw(menu.ui.is_selected(self.selection), brush, menu)
    }

    pub fn estimate_dimensions(&self, ui: &UI, mut width: isize) -> InternalWidgetDimensions {
        if width < 0 { width = 0; }
        // TODO: If it's 0, provide a stock answer

        let (last_width, last_dims) = self.last_dimensions.get();
        if last_width == width {
            return last_dims;
        }

        let new_dims = self.unique.estimate_dimensions(ui, width).fixup();
        self.last_dimensions.replace((last_width, new_dims));
        new_dims
    }

    pub fn apply_layout_hacks(&self, wd: WidgetDimensions) -> WidgetDimensions {
        self.unique.layout_hacks().apply(wd)
    }

    pub fn clear_layout_cache_if_needed(&self, ui: &UI) {
        if self.layout_token.get() < ui.layout_token() {
            self.last_dimensions.replace((-1, InternalWidgetDimensions::zero()));
            self.unique.clear_layout_cache(&ui);
            self.layout_token.replace(ui.layout_token());
        }
    }
}