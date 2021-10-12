use std::cell::{Ref, RefCell};

use chiropterm::{Brush, CellSize};
use euclid::{rect, size2};
use smallvec::SmallVec;

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
const SM: usize = 32;

pub type Column = Widget<ColumnState>;

pub struct ColumnState {
    widgets: SmallVec<[AnyWidget; SM]>,
    plots_desired: RefCell<(isize, (Plots, InternalWidgetDimensions))>,
    plots_practical: RefCell<(CellSize, Plots)>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for ColumnState {
    fn create() -> Self {
        ColumnState { 
            widgets: SmallVec::new(),
            plots_desired: RefCell::new((-1, (Plots::new(), InternalWidgetDimensions::zero()))),
            plots_practical: RefCell::new((size2(-1, -1), Plots::new())),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, ColumnState>) {
        let plots = self.get_plots_practical(&menu.ui, brush.rect().size);

        let mut total_y = 0;
        let width = brush.rect().width();
        for (w, p) in self.widgets.iter().zip(plots.1.plot_size.iter()) {
            let real_plot = brush.region(rect(0, total_y, width, *p));
            w.draw(real_plot.clone(), menu.share());
            total_y += p;
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        let plots = self.get_plots_desired(ui, width);
        plots.1.1
    }

    fn clear_layout_cache(&self, ui: &UI) {
        self.plots_desired.replace((-1, (Plots::new(), InternalWidgetDimensions::zero())));
        self.plots_practical.replace((size2(-1, -1), Plots::new()));
        for i in self.widgets.iter() {
            i.clear_layout_cache_if_needed(&ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl ColumnState {
    pub fn clear(&mut self) {
        self.widgets.clear()
    }

    pub fn add<X: Into<AnyWidget>>(&mut self, w: X) {
        self.widgets.push(w.into())
    }
}

impl ColumnState {
    fn get_plots_desired(&self, ui: &UI, width: isize) -> Ref<'_, (isize, (Plots, InternalWidgetDimensions))> {
        {
            let b = self.plots_desired.borrow();
            let (sz, (pl, _)) = &*b;
            if sz == &width && pl.plot_size.len() == self.widgets.len() {
                return b
            }
        }
        self.plots_desired.replace((width, self.internal_compute_plots_desired(&ui, width)));
        return self.plots_desired.borrow()
    }

    fn get_plots_practical(&self, ui: &UI, size: CellSize) -> Ref<'_, (CellSize, Plots)> {
        {
            let b = self.plots_practical.borrow();
            let (sz, pl) = &*b;
            if sz == &size && pl.plot_size.len() == self.widgets.len() {
                return b
            }
        }
        self.plots_practical.replace((size, self.internal_compute_plots_practical(ui, size)));
        return self.plots_practical.borrow()
    }
}

impl ColumnState {
    fn internal_compute_plots_desired(&self, ui: &UI, width: isize) -> (Plots, InternalWidgetDimensions) {
        // TODO: Use the cache
        let mut preferred: SmallVec<[isize; SM]> = SmallVec::new();

        let mut min_wmax = 0;
        let mut preferred_wmax = 0;

        let mut min_h = 0;
        let mut preferred_h = 0;

        let mut vertical_spacer_count = 0;
        // with no widgets: don't suddenly become a spacer
        // with widgets: be as much of a spacer as the widgets inside
        let mut horizontal_spacer_count = if self.widgets.len() > 0 { usize::MAX } else { 0 };

        for w in self.widgets.iter() {
            let dim = w.estimate_dimensions(ui, width);
            preferred.push(dim.preferred.height);

            min_wmax = min_wmax.max(dim.min.width);
            preferred_wmax = preferred_wmax.max(dim.preferred.width);

            min_h += dim.min.height;
            preferred_h += dim.preferred.height;

            vertical_spacer_count += dim.vertical_spacer_count;
            horizontal_spacer_count = horizontal_spacer_count.min(dim.horizontal_spacer_count);
        }
        assert_ne!(horizontal_spacer_count, usize::MAX);

        let dims = InternalWidgetDimensions {
            min: size2(min_wmax, min_h),
            preferred: size2(preferred_wmax, preferred_h),
            max: None,
            align_size_to: size2(1, 1),
            horizontal_spacer_count,
            vertical_spacer_count,
        };

        (Plots { plot_size: preferred }, dims)
    }

    fn internal_compute_plots_practical(&self, ui: &UI, size: CellSize) -> Plots {
        let mut likes_being_resized: SmallVec<[usize; SM]> = SmallVec::new();
        let mut minimum: SmallVec<[isize; SM]> = SmallVec::new();
        let mut practical: SmallVec<[isize; SM]> = SmallVec::new();
        let mut align: SmallVec<[isize; SM]> = SmallVec::new();

        for (w, widg) in self.widgets.iter().enumerate() {
            let dim = widg.estimate_dimensions(ui, size.width);
            for _ in 0..dim.vertical_spacer_count {
                likes_being_resized.push(w)
            }
            minimum.push(dim.min.height);
            practical.push(dim.preferred.height);
            align.push(dim.align_size_to.height);
        }

        if practical.len() == 0 || size.height < 0 { return Plots { plot_size: practical }; }

        let mut practical_sum: isize = practical.iter().sum();
        if practical_sum < size.height {
            // Expand whatever likes being resized
            if likes_being_resized.len() == 0 {
                // Just align to the top by expanding the bottom cell
                // (Basically, we're implicitly adding a spacer.)
                likes_being_resized.push(practical.len() - 1)
            }

            let og_rem = (size.height - practical_sum) as usize;
            let mut current_rem = og_rem;
            let portion = og_rem / likes_being_resized.len();
            for i in likes_being_resized.iter() {
                practical[*i] += portion as isize;
                current_rem -= portion;
            };
            for i in likes_being_resized.iter().take(current_rem as usize) {
                practical[*i] += 1;
            }
        }
        else {
            let mut desperate = false;

            'fix: while practical_sum > size.height {
                // Steal from everyone equally, starting at bottom
                let prev_sum = practical_sum;
                for i in (0..practical.len()).rev() {
                    if !desperate && practical[i] <= minimum[i] { continue }
                    if practical[i] <= 0 { continue }

                    practical[i] -= align[i];
                    practical_sum -= align[i];
                    if practical_sum <= size.height { 
                        break 'fix; 
                    }
                }
                if prev_sum == practical_sum { 
                    desperate = true; 
                }
            }
        }

        return Plots { plot_size: practical }
    }
}

struct Plots {
    plot_size: SmallVec<[isize; SM]>,
}

impl Plots {
    fn new() -> Plots {
        Plots { plot_size: SmallVec::new(), }
    }
}