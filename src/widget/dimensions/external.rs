use chiropterm::CellSize;

use super::InternalWidgetDimensions;

pub struct WidgetDimensions {
    pub min: CellSize,
    pub preferred: CellSize,
    pub align_size_to: CellSize,
    pub horizontal_spacer_count: usize,
    pub vertical_spacer_count: usize,
}

impl WidgetDimensions {
    pub(crate) fn to_internal(self) -> super::InternalWidgetDimensions {
        InternalWidgetDimensions {
            min: self.min,
            preferred: self.preferred,
            max: None,  // if we're using this function, we're nesting a widget that polices its own size
            align_size_to: self.align_size_to,
            horizontal_spacer_count: self.horizontal_spacer_count,
            vertical_spacer_count: self.vertical_spacer_count,
        }
    }
}