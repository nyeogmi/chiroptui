// TODO: Add scrollbar stuff to theme

use std::{cell::Cell};

use chiropterm::{Brush, FSem, Font, MouseButton, MouseEvent, Signal};
use euclid::{rect, vec2};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
pub type Scrollable = Widget<ScrollableState>;

pub struct ScrollableState {
    widget: Option<AnyWidget>,
    offset: Cell<f64>,

    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for ScrollableState {
    fn create() -> Self {
        ScrollableState { 
            widget: None,
            offset: Cell::new(0.0),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'frame, ScrollableState>) {
        if let Some(w) = &self.widget {
            let dims = w.estimate_dimensions(&menu.ui, brush.rect().width() - 2);

            let inner_height = dims.preferred.height;
            let brush_height = brush.rect().height();

            let offset_to_use = self.fix_offset(inner_height, brush_height);

            let space_to_adjust = (inner_height - brush_height).max(0);

            if space_to_adjust > 0 {
                let inner_width = brush.rect().width() - 2;

                let scrollbar = brush.region(rect(brush.rect().width() - 2, 0, 2, brush_height));

                let top_button = scrollbar.region(rect(0, 0, 2, 2));
                let btm_button = scrollbar.region(rect(0, scrollbar.rect().height() - 2, 2, 2));

                let scrollable_height = scrollbar.rect().height() - 4;

                let position_top = if space_to_adjust == 0 { 0.0 } else { offset_to_use as f64 / inner_height as f64 };
                let ix_top = (scrollable_height as f64 * position_top).floor() as isize;
                let barpart_height = if inner_height == 0 { 1 } else { 
                    (((brush_height as f64 / inner_height as f64) * scrollable_height as f64).ceil() as isize)
                    .max(1).min(scrollable_height) 
                };

                let scroll_offset_for = move |dy: f32| {
                    // TODO: This calculation is subtly wrong but I don't know why yet.
                    if scrollable_height == 0 { return 0.0; }
                    let scrolls_per_cell = inner_height as f64 / scrollable_height as f64;
                    dy as f64 * scrolls_per_cell
                };

                let mut ix_bot = ix_top + barpart_height;

                if ix_bot == ix_top { ix_bot += 1; }

                let scrollbar_rect = rect(0, ix_top + 2, 2, ix_bot - ix_top);

                let top_button_interactor = menu.on_click(move |_, w, me| {
                    match me {
                        MouseEvent::Click(MouseButton::Left, _, _) => { 
                            w.unique.set_offset(w.unique.offset.get() - scroll_offset_for(1 as f32).max(2.0), inner_height, brush_height); 
                            return Signal::Refresh;
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { .. } => {}
                        MouseEvent::Scroll(_, _, _) => {}
                    };
                    Signal::Continue
                });

                let btm_button_interactor = menu.on_click(move |_, w, me| {
                    match me {
                        MouseEvent::Click(MouseButton::Left, _, _) => { 
                            w.unique.set_offset(w.unique.offset.get() + scroll_offset_for(1 as f32).max(2.0), inner_height, brush_height); 
                            return Signal::Refresh;
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { .. } => {}
                        MouseEvent::Scroll(_, _, _) => {}
                    };
                    Signal::Continue
                });

                let bar_interactor = menu.on_click(move |_, w, me| {
                    // let me = me.offset(size2(scrollbar_rect.min_x() - brush.rect().min_x(), 0));
                    // may be no need, X doesn't matter
                    match me {
                        MouseEvent::Click(MouseButton::Left, point, _) => {
                            let scrollbar_center = (ix_top + ix_bot) / 2;
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for((point.y - scrollbar_center) as f32),
                                    inner_height, brush_height,
                            ); 
                            return Signal::Refresh;
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { 
                            mouse_button: MouseButton::Left,
                            last_point,
                            now_point,
                            .. 
                        } => {
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for((now_point.y - last_point.y) as f32),
                                inner_height, brush_height,
                            );
                            return Signal::Refresh;
                        }
                        MouseEvent::Drag { .. } => {} 
                        MouseEvent::Scroll(amt, _, _) => {
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for(amt),
                                inner_height, brush_height,
                            );
                            return Signal::Refresh;
                        }
                    }
                    Signal::Continue
                });

                let sb_brush = scrollbar.interactor(bar_interactor, menu.ui.theme().input_box.selected);
                sb_brush.fill(FSem::new().color(menu.ui.theme().input_box.deselected));
                sb_brush.bevel_w95(menu.ui.theme().input_box.bevel);

                let scrollbar_region = scrollbar.region(scrollbar_rect);
                scrollbar_region.bevel_w95(menu.ui.theme().button.bevel);
                scrollbar_region.interactor(bar_interactor, menu.ui.theme().input_box.cursor).fill(FSem::new().color(menu.ui.theme().input_box.cursor));

                top_button.bevel_w95(menu.ui.theme().button.bevel);
                btm_button.bevel_w95(menu.ui.theme().button.bevel);
                top_button.interactor(top_button_interactor, menu.ui.theme().button.preclick).font(Font::Set).putch(0x1eu16);
                btm_button.interactor(btm_button_interactor, menu.ui.theme().button.preclick).font(Font::Set).putch(0x1fu16);

                brush.dont_interfere_with_interactor().scroll_interactor(bar_interactor).fill(FSem::new());

                w.draw(
                    brush.region(
                        rect(0, 0, inner_width, dims.preferred.height.max(brush_height))
                    ).offset_rect(vec2(0, -offset_to_use)), 
                    menu.share()
                );
            } else {
                w.draw(
                    brush.region(
                        rect(0, 0, brush.rect().width(), dims.preferred.height.max(brush_height))
                    ).offset_rect(vec2(0, -offset_to_use)), 
                    menu.share()
                )
            }
        } 
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        if let Some(w) = &self.widget {
            let mut dims = w.estimate_dimensions(ui, width - 2).to_internal();
            dims.min.height = 4; // smallest capable of rendering a scrollbar, for now. consider an even smaller scrollbar later
            dims.min.width += 2;
            dims.preferred.width += 2;
            dims
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

impl ScrollableState {
    fn fix_offset(&self, inner_height: isize, brush_height: isize) -> isize {
        let space_to_adjust = (inner_height - brush_height).max(0);

        let new_offset = self.offset.get().max(0.0).min(space_to_adjust as f64);
        self.offset.replace(new_offset);
        let mut offset_to_use = self.offset.get() as isize;
        offset_to_use -= offset_to_use % 2;
        offset_to_use
    }

    fn set_offset(&self, new_value: f64, inner_height: isize, brush_height: isize) {
        self.offset.replace(new_value);
        self.fix_offset(inner_height, brush_height);
    }
}

impl ScrollableState {
    pub fn set<X: Widgetlike>(&mut self, w: Widget<X>) {
        self.widget = Some(AnyWidget::wrap(w))
    }
}
