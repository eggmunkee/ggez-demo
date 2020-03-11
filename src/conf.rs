use ggez::{ContextBuilder};
use ggez::conf::{WindowSetup,WindowMode,FullscreenType,NumSamples};

pub fn get_window_setup() -> WindowSetup {
    WindowSetup {
        title: "GGEZ ~~~ DEMO".to_owned(),
        samples: NumSamples::Two,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    }
}

pub fn get_window_mode() -> WindowMode {
    WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    }
}