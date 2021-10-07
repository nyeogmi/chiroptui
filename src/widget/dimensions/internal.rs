use chiropterm::*;
use euclid::{rect, size2};

use super::WidgetDimensions;

// TODO: "InternalWidgetDimensions" with an optional max and align
// ExternalWidgetDimensions with no max or align
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InternalWidgetDimensions {
    // widget will force its width below this if possible
    pub min: CellSize,
    pub preferred: CellSize,
    pub max: Option<CellSize>,
    pub align_size_to: CellSize,
    pub horizontal_spacer_count: usize,
    pub vertical_spacer_count: usize,
}

impl InternalWidgetDimensions {
    pub const fn zero() -> InternalWidgetDimensions {
        InternalWidgetDimensions {
            min: size2(0, 0),
            preferred: size2(0, 0),
            max: None,
            align_size_to: size2(1, 1),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    pub(crate) fn fixup(mut self) -> InternalWidgetDimensions {
        // TODO: fix impossibilities
        self = self.shape_to_align(self.align_size_to);
        self
    }

    pub fn tailor<'a>(&self, brush: Brush<'a>) -> Brush<'a> {
        let existing_size = brush.rect().size;

        let region = brush.region(rect(
            0, 0,
            existing_size.width - existing_size.width % self.align_size_to.width,
            existing_size.height - existing_size.height % self.align_size_to.height,
        ));

        let existing_size = brush.rect().size;

        let region = if let Some(max) = self.max {
            region.region(rect(
                0, 0,
                self.min.width.max(max.width.min(existing_size.width)),
                self.min.height.max(max.height.min(existing_size.height)),
            ))
        } else {
            region.region(rect(
                0, 0,
                self.min.width.max(existing_size.width),
                self.min.height.max(existing_size.height),
            ))
        };
        region
    }

    pub(crate) fn shape_to_align(mut self, align: CellSize) -> InternalWidgetDimensions {
        fn fix(align: CellSize, mut sz: CellSize) -> CellSize {
            if align.width > 0 && sz.width % align.width != 0 {
                sz.width += align.width - (sz.width % align.width);
            }
            if align.height > 0 && sz.height % align.height != 0 {
                sz.height += align.height - (sz.height % align.height);
            }
            sz
        }

        self.min = fix(align, self.min);
        self.preferred = fix(align, self.preferred);
        if let Some(max) = self.max {
            self.max = Some(fix(align, max));
        }

        self
    }

    pub(crate) fn increase(mut self, amt: CellSize) -> InternalWidgetDimensions {
        self.min.width += amt.width;
        self.min.height += amt.height;

        self.preferred.width += amt.width;
        self.preferred.height += amt.height;

        if let Some(mut max) = self.max {
            max.width += amt.width;
            max.height += amt.height;
            self.max = Some(max);
        }

        self
    }

    pub(crate) fn to_external(mut self) -> super::WidgetDimensions {
        self = self.fixup();
        WidgetDimensions {
            min: self.min,
            preferred: self.preferred,
            align_size_to: self.align_size_to,
            horizontal_spacer_count: self.horizontal_spacer_count,
            vertical_spacer_count: self.vertical_spacer_count,
        }
    }
}