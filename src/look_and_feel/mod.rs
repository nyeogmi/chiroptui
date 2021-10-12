mod w95;

pub use w95::W95Args;

#[derive(Clone, Copy)]
pub struct Theme {
    pub base: BaseTheme,
    pub window: WindowTheme,
    pub button: ButtonTheme,
    pub input_box: InputBoxTheme,
}

#[derive(Clone, Copy)]
pub struct BaseTheme {
    pub wallpaper: (u8, u8)
}

#[derive(Clone, Copy)]
pub struct WindowTheme {
    pub borders: WindowBorders,
    pub color: (u8, u8),
}

#[derive(Clone, Copy)]
pub enum WindowBorders {
    W95 { 
        bevel: (u8, u8),
        active_title: ([u8; 4], u8),
        inactive_title: ([u8; 4], u8),
    }, // TODO: Default title bar color?
    DOS { 
        border: u8,
        border_double: bool,
        active_title_fg: u8,
        inactive_title_fg: u8,
        // TODO: Shadow?
    },
}

#[derive(Clone, Copy)]
pub struct ButtonTheme {
    pub bevel: (u8, u8),
    pub color: (u8, u8),
    pub preclick: (u8, u8),
}

#[derive(Clone, Copy)]
pub struct InputBoxTheme {
    pub bevel: (u8, u8),
    pub deselected: (u8, u8),
    pub selected: (u8, u8),
    pub preclick: (u8, u8),
    pub cursor: (u8, u8),
}