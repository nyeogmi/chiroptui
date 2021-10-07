#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection(u64);

impl Selection {
    pub(in super) const fn none() -> Selection {
        Selection(!0u64)
    }

    pub const fn not_selected() -> Selection {
        // If you set your selection status to this, then the outer UI is guaranteed not to select you
        Selection(0)

    }

    pub(crate) fn advance(&self) -> Selection {
        if self.0 == Selection::none().0 {
            return Selection(1);
        }
        Selection(self.0 + 1)
    }
}