use chiropterm::{Brush};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

pub type Container = Widget<ContainerState>;

pub struct ContainerState {
    widget: Option<AnyWidget>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for ContainerState {
    fn create() -> Self {
        ContainerState { 
            widget: None,
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, ContainerState>) {
        if let Some(w) = &self.widget {
            w.draw(brush, menu);
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        if let Some(w) = &self.widget {
            w.estimate_dimensions(ui, width).to_internal()
        } else {
            return InternalWidgetDimensions::zero();
        }
    }

    fn clear_layout_cache(&self, ui: &UI) {
        if let Some(w) = &self.widget {
            w.clear_layout_cache_if_needed(ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { 
        self.layout_hacks 
    }
}

impl ContainerState {
    pub fn set<X: Into<AnyWidget>>(&mut self, w: X) {
        self.widget = Some(w.into())
    }
}
