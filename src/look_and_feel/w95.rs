use chiropterm::colors::*;

use super::*;

pub struct W95Args {
    wallpaper: (u8, u8),
    inset: (u8, u8),
    window: (u8, u8),
    accent_preclick: (u8, u8),
    accent_subselected: (u8, u8),
    title_bar: ([u8; 4], u8),

    // "enclaves" include text boxes
    enclave: (u8, u8),
}

impl Theme {
    /*
    pub const W95: Theme = Theme::w95(
        W95Args {
            wallpaper: (LtGreen[0], Dark[0]),
            inset: (Dark[0], Light[3]),
            window: (Light[2], Dark[0]),
            accent_preclick: (LtGreen[2], Dark[0]),
            accent_subselected: (LtOrange[2], Dark[0]),
            enclave: (Light[3], Dark[0]),
        }
    );

    pub const W95_DARK: Theme = Theme::w95(
        W95Args {
            wallpaper: (LtOrange[0], Dark[0]),
            inset: (Dark[0], Light[0]),
            window: (Dark[1], Light[2]),
            accent_preclick: (LtBlue[0], Light[2]),
            accent_subselected: (LtOrange[2], Light[2]),

            enclave: (Dark[0], Light[2]),
        }
    );
    */

    pub const W95_FRUITY: Theme = Theme::w95_fruity(
        (Dark[2], Dark[0]),
        // DkGreen, LtGreen, LtYellow,
        DkPurple, LtPurple, LtFuchsia,
        White,
        // good for green and blue
        // LtYellow[1], LtOrange[1],
        // good for fuchsia 
        // DkPurple[0], LtPurple[1],
        // good for purple 
        DkPurple[0], LtFuchsia[1],
        // good for Dark/Light and cyan
        // DkRed[0], LtRed[1],
    );

    pub const fn w95_fruity(
        wallpaper: (u8, u8), 
        dark: [u8; 4], 
        light: [u8; 4], 
        title_bar: [u8; 4],
        lightest: u8,  // recommended: White
        accent_preclick: u8, // recommended: LtX[1]
        accent_subselected: u8, // recommended: LtX[1]
    ) -> Theme {
        Theme::w95(W95Args {
            wallpaper,
            inset: (dark[2], light[1]),
            window: (light[0], light[3]),
            accent_preclick: (accent_preclick, lightest),
            accent_subselected: (accent_subselected, lightest),
            title_bar: (title_bar, lightest),
            enclave: (dark[3], light[3]),
        })
    }

    pub const fn w95(args: W95Args) -> Theme {
        let wallpaper = args.wallpaper;
        let inset = args.inset;
        let outset = (inset.1, inset.0);
        let window = args.window;
        let accent_preclick = args.accent_preclick;
        let accent_subselected = args.accent_subselected;
        let title_bar = args.title_bar;
        let enclave = args.enclave;

        Theme {
            base: BaseTheme { 
                wallpaper: wallpaper
            },
            window: WindowTheme {
                borders: WindowBorders::W95 { 
                    bevel: outset,
                    active_title: title_bar,
                    inactive_title: (Light, Light[3]),
                },
                color: window,
            },
            button: ButtonTheme { 
                bevel: outset,
                color: window,  // TODO: This is completely wrong!
                preclick: accent_preclick,
            },
            input_box: InputBoxTheme {
                bevel: inset,
                deselected: enclave,
                selected: enclave,
                preclick: accent_preclick,
                cursor: accent_subselected,
            },
        }
    }
}