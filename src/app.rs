pub struct AppState {
    // Menu
    pub open_menu: Option<&'static str>,
}

impl AppState {
    pub fn new() -> Self {
        Self { open_menu: None }
    }
}
