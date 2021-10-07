use std::cell::{Ref, RefCell};

use chiropterm::{Brush, Brushable, Stamp};
use euclid::{rect, size2};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Label = Widget<LabelState>;

pub struct LabelState {
    text: String,
    stamp: RefCell<(isize, Stamp)>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for LabelState {
    fn create() -> Self {
        Self {
            text: "".to_owned(),
            stamp: RefCell::new((-1, Stamp::new())),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, _menu: WidgetMenu<'frame, Self>) {
        let stamp = self.stamp(brush.rect().width());
        stamp.1.draw(brush);
    }

    fn estimate_dimensions(&self, _: &UI, width: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do the measurement
        let stamp = self.stamp(width);

        InternalWidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.1.rect().size,
            // TODO: Better foundation for this number
            max: None,
            align_size_to: size2(1, 2),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl LabelState {
    fn stamp(&self, width: isize) -> Ref<(isize, Stamp)> {
        {
            let s = self.stamp.borrow();
            if s.0 == width {
                return s
            }
        }

        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        self.stamp.replace((width, stamp));
        self.stamp.borrow()
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.stamp.replace((-1, Stamp::new()));
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}