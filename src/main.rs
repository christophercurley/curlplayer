pub mod curl_ui;

use macroquad::prelude::*;

use crate::curl_ui::{build_menu_bar, render_ui};

// ----------------------------------------------
// Component Store
// ----------------------------------------------

enum Shape {
    Rect { x: f32, y: f32, w: f32, h: f32 },
}

struct Component {
    name: &'static str,
    shape: Shape,
}

pub struct ComponentStore {
    components: Vec<Component>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.components.clear();
    }
}

// ----------------------------------------------
// Window Conf
// ----------------------------------------------

fn window_conf() -> Conf {
    Conf {
        window_title: "curlplayer".to_owned(),
        window_width: 1280,
        window_height: 720,
        // window_resizable: false,
        // fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

// ----------------------------------------------
// Main
// ----------------------------------------------

#[macroquad::main(window_conf)]
async fn main() {
    let font =
        load_ttf_font_from_bytes(include_bytes!("../assets/SourceSans3-Regular.ttf")).unwrap();
    let menu_font_size: u16 = 18;
    let menu_bar = build_menu_bar();

    let mut component_store = ComponentStore::new();

    loop {
        render_ui(&menu_bar, &font, menu_font_size, &mut component_store);

        next_frame().await;
    }
}
