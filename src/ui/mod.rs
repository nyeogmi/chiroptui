mod selection;

use std::{cell::Cell, rc::Rc};

pub use self::selection::Selection;

use super::{Theme, WidgetCommon, Widgetlike};

pub struct UISource {
    selection: Cell<Selection>,
    layout_token: Cell<u64>,
    theme: Cell<Theme>,
}

#[derive(Clone)]
pub struct UI {
    state: Rc<UISource>, 
    context: UIContext,
}

impl UI {
    pub fn new(theme: Theme) -> UI {
        UI {
            state: Rc::new(UISource { 
                selection: Cell::new(Selection::none()),
                layout_token: Cell::new(0),
                theme: Cell::new(theme),
            }),
            context: UIContext::new(),
        }
    }

    pub fn share(&self) -> UI {
        UI { state: self.state.clone(), context: self.context }
    }

    pub fn theme(&self) -> Theme {
        self.state.theme.get()
    }

    pub fn select<T: Widgetlike>(&self, widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
        widg.selection = self.state.selection.get();
    }

    pub fn deselect<T: Widgetlike>(&self, _widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
    }

    pub fn is_selected(&self, other: Selection) -> bool {
        self.state.selection.get() == other
    }

    pub fn recompute_layout(&self) {
        self.state.layout_token.replace(self.state.layout_token.get() + 1);
    }

    pub(in crate) fn layout_token(&self) -> u64 {
        self.state.layout_token.get()
    }

    pub fn with_context(mut self, on_ctx: impl FnOnce(&mut UIContext)) -> UI {
        on_ctx(&mut self.context);
        self
    }

    pub fn context(&self) -> UIContext {
        self.context
    }
}

#[derive(Clone, Copy)]
pub struct UIContext {
    pub active: bool,
}

impl UIContext {
    pub fn new() -> UIContext {
        UIContext {
            active: true,
        }
    }
}