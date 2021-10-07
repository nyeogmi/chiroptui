use chiropterm::{Brush};
use euclid::{size2};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Spacer = Widget<SpacerState>;

pub struct SpacerState {
    pub horiz_count: usize,
    pub vert_count: usize,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for SpacerState {
    fn create() -> Self {
        Self {
            horiz_count: 1,
            vert_count: 1,

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, _: Brush, _: WidgetMenu<'frame, Self>) {
    }

    fn estimate_dimensions(&self, _: &UI, _: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do this
        InternalWidgetDimensions {
            min: size2(0, 0),
            preferred: size2(0, 0),
            max: None,
            align_size_to: size2(1, 1),
            horizontal_spacer_count: 1,
            vertical_spacer_count: 1,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}