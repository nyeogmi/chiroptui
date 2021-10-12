use chiropterm::{Brush};
use euclid::{rect, size2};
use smallvec::SmallVec;

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
const SM: usize = 32;

pub type Deck = Widget<DeckState>;

pub struct DeckState {
    widgets: SmallVec<[AnyWidget; SM]>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for DeckState {
    fn create() -> Self {
        DeckState { 
            widgets: SmallVec::new(),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, DeckState>) {
        let top = if let Some(top) = self.widgets.last() {
            top
        } else { return };

        let dims = top.estimate_dimensions(&menu.ui, brush.rect().width());
        let preferred_height = dims.preferred.height;

        let mut y_tar = (brush.rect().height() - preferred_height).max(0);
        y_tar -= y_tar % 2;

        // don't let the widgets push them down too much
        if y_tar > (self.widgets.len() * 2) as isize {
            y_tar = (self.widgets.len() * 2) as isize;
        }

        top.draw(
            brush.region(rect(0, y_tar, brush.rect().width(), brush.rect().max_y() - y_tar)),
            menu.share(),
        );

        let mut ix = self.widgets.len() - 1;
        loop {
            y_tar -= 2;
            ix -= 1;

            if y_tar < 0 { unreachable!(); }

            self.widgets[ix].draw(
                brush.region(rect(0, y_tar, brush.rect().width(), 2)), 
                menu.share().with_context(|ctx| ctx.active = false),
            );

            if ix == 0 { 
                break; 
            }
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        let mut max_min_w = 0;
        let mut max_preferred_w = 0;

        for i in self.widgets.iter() {
            let dims = i.estimate_dimensions(ui, width);
            max_min_w = max_min_w.max(dims.min.width);
            max_preferred_w = max_preferred_w.max(dims.preferred.width);
        }

        let mut min_h = ((self.widgets.len() as isize - 1) * 2).max(0);
        let mut preferred_h = min_h;
        if let Some(w) = self.widgets.last() {
            let dims = w.estimate_dimensions(ui, width);
            min_h += dims.min.height;
            preferred_h += dims.preferred.height;
        }

        return InternalWidgetDimensions {
            min: size2(max_min_w, min_h),
            preferred: size2(max_preferred_w, preferred_h),
            max: None,
            align_size_to: size2(1, 1),  // TODO: (1, 2)?
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, ui: &UI) {
        for i in self.widgets.iter() {
            i.clear_layout_cache_if_needed(&ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl DeckState {
    pub fn add<X: Into<AnyWidget>>(&mut self, w: X) {
        self.widgets.push(w.into())
    }
}
