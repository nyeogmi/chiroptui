use std::{cell::{Ref, RefCell}};

use chiropterm::{Brush, CellSize};
use euclid::{rect, size2};
use smallvec::SmallVec;

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
const SM: usize = 32;

pub type Row = Widget<RowState>;

pub struct RowState {
    widgets: SmallVec<[AnyWidget; SM]>,
    plots_desired: RefCell<(isize, (Plots, InternalWidgetDimensions))>,
    plots_practical: RefCell<(CellSize, Plots)>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for RowState {
    fn create() -> Self {
        RowState { 
            widgets: SmallVec::new(),
            plots_desired: RefCell::new((-1, (Plots::new(), InternalWidgetDimensions::zero()))),
            plots_practical: RefCell::new((size2(-1, -1), Plots::new())),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, RowState>) {
        let plots = self.get_plots_practical(&menu.ui, brush.rect().size);

        let mut total_x = 0;
        let height = brush.rect().height();
        for (w, p) in self.widgets.iter().zip(plots.1.plot_size.iter()) {
            let real_plot = brush.region(rect(total_x, 0, *p, height));
            w.draw(real_plot.clone(), menu.share());
            total_x += p;
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

impl RowState {
    pub fn add<X: Into<AnyWidget>>(&mut self, w: X) {
        self.widgets.push(w.into())
    }
}

impl RowState {
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

impl RowState {
    fn internal_compute_plots_desired(&self, ui: &UI, width: isize) -> (Plots, InternalWidgetDimensions) {
        let mut preferred: SmallVec<[isize; SM]> = SmallVec::new();

        let mut min_hmax = 0;
        let mut preferred_hmax = 0;

        let mut min_w = 0;
        let mut preferred_w = 0;

        let mut horizontal_spacer_count = 0;
        // with no widgets: don't suddenly become a spacer
        // with widgets: be as much of a spacer as the widgets inside
        let mut vertical_spacer_count = if self.widgets.len() > 0 { usize::MAX } else { 0 };

        for w in self.widgets.iter() {
            let dim = w.estimate_dimensions(ui, width);
            preferred.push(dim.preferred.width);

            min_hmax = min_hmax.max(dim.min.height);
            preferred_hmax = preferred_hmax.max(dim.preferred.height);

            min_w += dim.min.width;
            preferred_w += dim.preferred.width;

            horizontal_spacer_count += dim.horizontal_spacer_count;
            vertical_spacer_count = vertical_spacer_count.min(dim.vertical_spacer_count);
        }
        assert_ne!(vertical_spacer_count, usize::MAX);

        let dims = InternalWidgetDimensions {
            min: size2(min_w, min_hmax),
            preferred: size2(preferred_w, preferred_hmax),
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
            for _ in 0..dim.horizontal_spacer_count {
                likes_being_resized.push(w)
            }
            minimum.push(dim.min.width);
            practical.push(dim.preferred.width);
            align.push(dim.align_size_to.width);
        }

        if practical.len() == 0 || size.width < 0 { return Plots { plot_size: practical }; }

        let mut practical_sum: isize = practical.iter().sum();
        if practical_sum < size.width {
            // Expand whatever likes being resized
            if likes_being_resized.len() == 0 {
                // Expand every cell
                likes_being_resized.extend(0..self.widgets.len())
            }

            let og_rem = (size.width - practical_sum) as usize;
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

            'fix: while practical_sum > size.width {
                // Steal from everyone equally, starting at bottom
                let prev_sum = practical_sum;
                for i in (0..practical.len()).rev() {
                    if !desperate && practical[i] <= minimum[i] { continue }
                    if practical[i] <= 0 { continue }

                    practical[i] -= align[i];
                    practical_sum -= align[i];
                    if practical_sum <= size.width { 
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