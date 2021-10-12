use chiropterm::*;
use euclid::{rect, size2};

use crate::{InternalWidgetDimensions, UI, Widget, WidgetCommon, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Button = Widget<ButtonState>;

// TODO: Hotkeys
pub struct ButtonState {
    pub hotkey: Option<Keycode>,
    pub text: String,
    pub command: Option<Box<dyn FnMut(UI, &mut WidgetCommon<ButtonState>, InputEvent) -> Signal>>,

    pub layout_hacks: LayoutHacks,
}

impl ButtonState {
    pub fn set_command(&mut self, cmd: impl 'static+FnMut(UI, &mut WidgetCommon<ButtonState>, InputEvent) -> Signal) {
        self.command = Some(Box::new(cmd))
    }
}

impl Widgetlike for ButtonState {
    fn create() -> Self {
        Self {
            hotkey: None,
            text: "".to_owned(),
            command: None,

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'frame, Self>) {
        let click_interactor = menu.on_mouse(move |ui, this, click: MouseEvent| {
            match click {
                MouseEvent::Click(_, _, _) => { 
                    return ButtonState::click(ui, this, InputEvent::Mouse(click));
                },
                MouseEvent::Up(_, _, _) => {}
                MouseEvent::Drag {..} => {}
                MouseEvent::Scroll(_, _, _) => {}
                MouseEvent::Wiggle {..} => {}
            };
            Signal::Refresh
        });
        
        if let Some(hotkey) = self.hotkey {
            menu.on_key(OnKey::only(hotkey).pressed(), move |ui, this, key| {
                ButtonState::click(ui, this, InputEvent::Keyboard(key))
            });
        }

        let theme = menu.ui.theme().button;
        brush.bevel_w95(theme.bevel);
        brush.interactor(click_interactor, theme.preclick).putfs(&self.text);
    }

    fn estimate_dimensions(&self, _ui: &UI, width: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do this
        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        InternalWidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.rect().size,
            // TODO: Better foundation for this number
            max: Some(size2(self.text.len() as isize, 2)),
            align_size_to: size2(1, 2),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl ButtonState {
    fn click(ui: UI, this: &mut WidgetCommon<Self>, input: InputEvent) -> Signal {
        ui.select(this); // this button can be selected, not that it matters. just deselect other stuff
        let command = this.unique.command.take();
        if let Some(mut c) = command {
            let result = c(ui, this, input);
            this.unique.command.replace(c);
            return result
        }
        Signal::Refresh
    }
}