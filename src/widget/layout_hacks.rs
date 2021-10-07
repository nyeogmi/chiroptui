use super::WidgetDimensions;

#[derive(Clone, Copy)]
pub struct LayoutHacks {
    pub expand_horizontally: bool,
    pub expand_vertically: bool,

    // NYEO NOTE: It's completely OK for these to be settable to arbitrary values because the min/max constraints will be handled in tailor()
    pub preferred_width: Option<usize>,
    pub preferred_height: Option<usize>,
}

impl LayoutHacks {
    pub fn new() -> LayoutHacks {
        LayoutHacks {
            expand_horizontally: false,
            expand_vertically: false,

            preferred_width: None,
            preferred_height: None,
        }
    }

    pub fn apply(&self, mut wd: WidgetDimensions) -> WidgetDimensions {
        if self.expand_horizontally { 
            wd.horizontal_spacer_count = wd.horizontal_spacer_count.max(1); 
        };
        if self.expand_vertically { 
            wd.vertical_spacer_count = wd.vertical_spacer_count.max(1); 
        };
        if let Some(w) = self.preferred_width {
            wd.preferred.width = w as isize;
            wd.min.width = wd.min.width.min(w as isize);
        }
        if let Some(h) = self.preferred_height {
            wd.preferred.height = h as isize;
            wd.min.height = wd.min.height.min(h as isize);
        }
        wd
    }
}