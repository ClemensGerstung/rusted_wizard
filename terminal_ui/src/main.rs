mod ui_app;
mod player_count_input_popup;
mod player_name_input_popup;

use rusted_wizard_core;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyModifiers;
use rusted_wizard_core::{Wizard, WizardState};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = ui_app::App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: ui_app::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        match app.game.as_mut() {
            Some(wizard) => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    }

                    if wizard.state == WizardState::Init {
                        match key.code {
                            KeyCode::Tab => {
                                let index = if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    (app.player_name_index - 1) % wizard.player_count
                                } else {
                                    (app.player_name_index + 1) % wizard.player_count
                                };
                                app.player_name_index = index;
                            }
                            KeyCode::Char(c) => {
                                app.player_names[app.player_name_index].push(c);
                            }
                            KeyCode::Backspace => {
                                app.player_names[app.player_name_index].pop();
                            }
                            KeyCode::Enter => {
                                let is_ok = app.player_names.iter().all(|pn| !pn.is_empty());
                                if is_ok {
                                    for _ in 0..wizard.player_count {
                                        wizard.play(|player_index| {
                                            app.player_names[player_index].clone()
                                        },
                                                    |_, _| { 0 });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    else {

                    }
                }
            }
            None => {
                if app.player_count.is_empty() {
                    app.hint = String::from("Number of players required");
                } else {
                    app.hint = String::new();
                }

                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            match app.player_count.parse::<usize>() {
                                Ok(player_count) => {
                                    app.game = Option::from(Wizard::new(player_count));
                                    for _ in 0..player_count {
                                        app.player_names.push(String::new());
                                    }
                                }
                                Err(_) => {
                                    app.hint = String::from("No valid input!")
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            let mut temp = String::from(&app.player_count);
                            temp.push(c);

                            match temp.parse::<usize>() {
                                Ok(val) => {
                                    if val >= 3 && val <= 6 {
                                        app.player_count.push(c);
                                        app.hint = String::new();
                                    } else {
                                        app.hint = String::from("Player count must be between (including) 3 and 6");
                                    }
                                }
                                Err(_) => {
                                    app.hint = String::from("Not a Number");
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            app.player_count.pop();
                        }
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &ui_app::App) {
    let size = f.size();

    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue));
    f.render_widget(block, size);

    player_count_input_popup::draw(f, app);
    player_name_input_popup::draw(f, app);
}
