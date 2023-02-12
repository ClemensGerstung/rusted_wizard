mod player_count_input_popup;
mod player_name_input_popup;
mod ui_app;
mod playground;

use rusted_wizard_core;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crossterm::event::KeyModifiers;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusted_wizard_core::{Wizard, WizardState};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = ui_app::App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut ui_app::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(());
            }

            player_name_input_popup::handle_input(app, key);
            player_count_input_popup::handle_input(app, key);
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &ui_app::App) {
    let block = Block::default()
        .style(Style::default().bg(Color::Rgb(50, 50, 50)));
    f.render_widget(block, f.size());

    player_count_input_popup::draw(f, app);
    player_name_input_popup::draw(f, app);
    playground::draw(f, app);
}
