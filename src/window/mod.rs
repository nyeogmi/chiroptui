use chiropterm::*;
use euclid::{rect, size2};

use super::{InternalWidgetDimensions, Label, UI, Widget, WidgetMenu, Widgetlike, look_and_feel::WindowBorders, widget::{AnyWidget, LayoutHacks}};

pub type Window = Widget<WindowState>;

// TODO: Support a w95-ish border type too

pub struct WindowState {
    title: Option<Label>,
    title_text: Option<String>,  // all labels are potentially shared, so we have to clone it to provide a getter
    widget: Option<AnyWidget>,

    pub window_border_override: Option<WindowBorders>,
    pub layout_hacks: LayoutHacks,
}

impl Widgetlike for WindowState {
    fn create() -> Self {
        WindowState { 
            title: None,
            title_text: None,
            widget: None,

            window_border_override: None,
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'frame, Self>) {
        brush.fill(FSem::new().color(menu.ui.theme().window.color));

        let borders = self.window_border_override.unwrap_or(menu.ui.theme().window.borders);
        let inner = match borders {
            WindowBorders::W95 { bevel, active_title: active_title_color, inactive_title: inactive_title_color } => {
                brush.bevel_w95(bevel);

                let title_color = if menu.ui.context().active {
                    active_title_color
                } else {
                    inactive_title_color
                };

                if let Some(title) = &self.title {
                    let title_bar_outer = rect(0, 0, brush.rect().size.width, 2);
                    let title_bar_inner = rect(1, 0, brush.rect().size.width - 2, 2);

                    // get rid of w95 gradients around the title bar
                    brush.region(title_bar_outer).bevel_bottom(255);
                    brush.region(title_bar_outer).bevel_left(255);
                    brush.region(title_bar_outer).bevel_top(255);

                    draw_gradient(brush.region(title_bar_outer), [title_color.0[1], title_color.0[2]]);

                    let title_brush = brush.region(title_bar_inner).fg(title_color.1);
                    title.draw(menu.ui.share(), title_brush, menu.menu.share());

                    let inner = rect(1, 2, brush.rect().size.width - 2, brush.rect().size.height - 3);
                    brush.region(inner)
                }
                else {
                    brush.region(brush.rect().inflate(-1, -1))
                }

                // TODO: Use title
            }
            WindowBorders::DOS { 
                border, border_double,
                active_title_fg,
                inactive_title_fg,
            } => {
                let title_color = if menu.ui.context().active {
                    active_title_fg
                } else {
                    inactive_title_fg
                };

                brush.fg(border).draw_box(border_double);  // TODO. Box and box color in theme

                if let Some(title) = &self.title {
                    let title_brush = brush.region(rect(2, 0, brush.rect().size.width - 4, 2)).fg(title_color);
                    title.draw(menu.ui.share(), title_brush, menu.menu.share());
                }

                brush.region(brush.rect().inflate(-2, -2))
            }
        };

        match &self.widget {
            Some(x) => x.draw(inner, menu),
            None => {}
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        let borders = self.window_border_override.unwrap_or(ui.theme().window.borders);
        let ((pad_x, pad_y), (align_x, align_y)) = match borders {
            WindowBorders::W95 { .. } => {
                if self.title.is_some() {
                    ((2, 3), (1, 1))
                } else {
                    ((2, 2), (1, 1))
                }
            }
            WindowBorders::DOS { .. } => {
                ((4, 4), (2, 2))
            }
        };
        let d1 = if let Some(w) = self.widget.as_ref() {
            w.estimate_dimensions(ui, width - pad_x)
        } else {
            InternalWidgetDimensions::zero().to_external()
        };

        let mut size = InternalWidgetDimensions { 
            min: d1.min,
            preferred: d1.preferred,
            max: None,
            align_size_to: size2(align_x, align_y),
            // don't pass spacers through
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0
        };
        size = size.increase(size2(pad_x, pad_y));
        size
    }

    fn clear_layout_cache(&self, ui: &UI) { 
        if let Some(w) = self.widget.as_ref() {
            w.clear_layout_cache_if_needed(ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl WindowState {
    pub fn set<X: Into<AnyWidget>>(&mut self, w: X) {
        self.widget = Some(w.into())
    }
}

// TODO: Do a title bar theme that uses code similar to this to just do a Powerline bar
// Maybe just have two or three powerline bars to the far right, near the window buttons
fn draw_gradient(brush: Brush, color_opts: [u8; 2]) {
    let glyph_opts = [(false, 0), (false, 0xb0), (false, 0xb1), /* (true, 0xb0) */];  // commented out for trypophobia reasons for now
    let glyph_opts_2 = [(false, 0xb0), (false, 0xb1), (false, 0xdb) /* (true, 0xb0) */];  // commented out for trypophobia reasons for now
    let n_opts = color_opts.len() * glyph_opts.len() - (glyph_opts.len() - 1);
    let max_x_offset = 1;
    for (y0, h, x_offset) in [(0, 1, 0), (1, 1, -1)] {
        for i in 0..n_opts as isize {
            let x0 = (i * brush.rect().size.width + max_x_offset) / n_opts as isize + x_offset;
            let x1 = ((i + 1) * brush.rect().size.width + max_x_offset) / n_opts as isize + x_offset;
            let r = rect(x0, y0, x1 - x0, h);
            let rend = rect(x1 - 1, y0, x1, h);

            let color_ix = (i/glyph_opts.len() as isize) as usize;
            let last_color = color_opts[color_ix];
            let next_color = color_opts[(color_ix + 1).min(color_opts.len() - 1)];
            let (flip, c1) = glyph_opts[i as usize % glyph_opts.len()];
            let (_, c2) = glyph_opts_2[i as usize % glyph_opts.len()];

            let (bg, fg) = if flip { (next_color, last_color) } else { (last_color, next_color) };

            brush.region(r).fill(FSem::new().bg(bg).fg(fg).sem(SemanticContent::Small(c1)));
            brush.region(rend).fill(FSem::new().bg(bg).fg(fg).sem(SemanticContent::SmallPizza1(c1, c2)))
        }
    }
}

impl WindowState {
    pub fn set_title(&mut self, title: impl Into<String>) {
        let s = title.into();
        self.title_text = Some(s.clone());
        if let Some(widg) = &self.title {
            widg.setup(|w| w.set_text(s));
        } else {
            self.title = Some(Label::new().setup(|w| w.set_text(s)));
        }
    }

    pub fn get_title(&self) -> Option<&str> {
        self.title_text.as_ref().map(|t| &t[..])
    }
}