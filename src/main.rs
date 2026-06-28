pub mod app;
pub mod curl_ui;
pub mod primitives;

use crate::app::AppState;
use crate::curl_ui::{build_menu_bar, render_ui};
use macroquad::prelude::*;

// ----------------------------------------------
// Component Store
// ----------------------------------------------
#[derive(Debug)]
enum Shape {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    RectRE {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        radius: f32,
    },
}

#[derive(Debug, PartialEq)]
enum Group {
    Debug,
    MenuBar,
    MenuBarPanel,
}

#[derive(Debug)]
struct Component {
    name: &'static str,
    group: Group,
    shape: Shape,
}

#[derive(Debug)]
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

    fn add(&mut self, component: Component) {
        self.components.push(component);
    }
}

impl Component {
    pub fn new(name: &'static str, group: Group, shape: Shape) -> Self {
        Self { name, shape, group }
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
        sample_count: 4,
        high_dpi: true,
        ..Default::default()
    }
}

// ----------------------------------------------
// Main
// ----------------------------------------------

#[macroquad::main(window_conf)]
async fn main() {
    let mut font =
        load_ttf_font_from_bytes(include_bytes!("../assets/SourceSans3-Regular.ttf")).unwrap();
    font.set_filter(FilterMode::Nearest);
    let menu_font_size: u16 = 18;

    let mut app_state = AppState::new();
    let menu_bar = build_menu_bar();

    let mut component_store = ComponentStore::new();

    loop {
        render_ui(
            &menu_bar,
            &font,
            menu_font_size,
            &mut component_store,
            &mut app_state,
        );

        next_frame().await;
    }
}
