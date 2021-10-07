use chiropterm::{Brush};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Canvas = Widget<CanvasState>;

pub struct CanvasState {
    pub layout_hacks: LayoutHacks,
    pub draw: Option<Box<dyn for<'frame> Fn(Brush, WidgetMenu<'frame, CanvasState>)>>,
}

impl CanvasState {
    pub fn set_draw(&mut self, draw: impl 'static+for<'frame> Fn(Brush, WidgetMenu<'frame, CanvasState>)) {
        self.draw = Some(Box::new(draw))
    }
}

impl Widgetlike for CanvasState {
    fn create() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
            draw: None,
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'frame, Self>) {
        if let Some(d) = &self.draw {
            d(brush, menu)
        }
    }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> InternalWidgetDimensions {
        // let the client use layout hacks
        InternalWidgetDimensions::zero()
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}