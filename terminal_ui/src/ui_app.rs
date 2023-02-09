pub struct App {
    pub game: Option<rusted_wizard_core::Wizard>,
    pub player_count: String,
    pub player_names: Vec<String>,
    pub player_name_index: usize,
    pub hint: String,
}

impl App {
    pub fn new() -> App {
        App {
            game: None,
            player_count: String::new(),
            player_names: vec![],
            player_name_index: 0,
            hint: String::new(),
        }
    }
}
