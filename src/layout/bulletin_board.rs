use chiropterm::{Brush, CellPoint};
use euclid::{rect, size2};
use smallvec::SmallVec;

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
const SM: usize = 32;

pub type BulletinBoard = Widget<BulletinBoardState>;

pub struct BulletinBoardState {
    widgets: SmallVec<[(CellPoint, AnyWidget); SM]>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for BulletinBoardState {
    fn create() -> Self {
        BulletinBoardState { 
            widgets: SmallVec::new(),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, BulletinBoardState>) {
        for (offset, w) in &self.widgets {
            let dims = w.estimate_dimensions(&menu.share().ui, brush.rect().width());
            let x = offset.x.min(brush.rect().width() - dims.preferred.width).max(0);
            let y = offset.y.min(brush.rect().height() - dims.preferred.height).max(0);

            w.draw(brush.region(rect(
                x, y, 
                dims.preferred.width.min(brush.rect().width() - x),
                dims.preferred.height.min(brush.rect().height() - y),
            )), menu.share())
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        let mut max_min_x = 0;
        let mut max_min_y = 0;

        let mut max_pref_x = 0;
        let mut max_pref_y = 0;

        for (offset, w) in &self.widgets {
            let dims = w.estimate_dimensions(ui, width - offset.x);

            // at minimum, widgets will be edged onto the screen from outside
            max_min_x = max_min_x.max(dims.min.width);
            max_min_y = max_min_y.max(dims.min.height);

            max_pref_x = max_pref_x.max(dims.preferred.width + offset.x);
            max_pref_y = max_pref_y.max(dims.preferred.height + offset.y);
        }

        return InternalWidgetDimensions {
            min: size2(max_min_x, max_min_y),
            preferred: size2(max_pref_x, max_pref_y),
            max: None,
            align_size_to: size2(1, 1), 
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, ui: &UI) {
        for (_, w) in self.widgets.iter() {
            w.clear_layout_cache_if_needed(&ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl BulletinBoardState {
    pub fn add<X: Into<AnyWidget>>(&mut self, at: CellPoint, w: X) {
        self.widgets.push((at, w.into()))
    }
}
