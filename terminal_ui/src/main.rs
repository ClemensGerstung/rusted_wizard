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
use rusted_wizard_core::Wizard;

struct App {
    game: Option<rusted_wizard_core::Wizard>,
    show_popup: bool,
    player_count: String,
    hint: String,
}

impl App {
    fn new() -> App {
        App {
            game: Option::None,
            show_popup: false,
            player_count: String::new(),
            hint: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        match &app.game {
            Some(wizard) => {


            },
            None => {
                app.show_popup = true;

                if app.player_count.is_empty() {
                    app.hint = String::from("Number of players required");
                } else {
                    app.hint = String::new();
                }

                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            let player_count = app.player_count.parse::<usize>().unwrap();

                            app.game = Option::from(Wizard::new(player_count));
                            app.show_popup = false;
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
                                },
                                Err(_) => {
                                    app.hint = String::from("Not a Number");
                                }
                            }
                        },
                        KeyCode::Backspace => {
                            app.player_count.pop();
                        },
                        KeyCode::Esc => {
                            return Ok(());
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Blue));
    f.render_widget(block, size);

    if app.show_popup {
        render_player_count_popup(f, &app.player_count, &app.hint);
    }
}

fn render_player_count_popup<'a, B: Backend>(f: &mut Frame<B>, player_count: &'a String, hint: &'a String) {
    let size = f.size();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ].as_ref(),
        )
        .split(size);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
                .as_ref(),
        );
    let text_area = horizontal_layout.split(popup_layout[1])[1];
    let hint_area = horizontal_layout.split(popup_layout[2])[1];

    f.render_widget(Clear, text_area); //this clears out the background
    f.render_widget(Clear, hint_area); //this clears out the background

    let input = Paragraph::new(player_count.clone())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("How many Players?"));
    f.render_widget(input, text_area);
    f.set_cursor(
        text_area.x + player_count.width() as u16 + 1,
        text_area.y + 1,
    );

    if !hint.is_empty() {
        let text = vec![
            Span::from(Span::styled(
                "Hint: ",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )),
            Span::from(hint.clone()),
        ];

        let hint = Block::default().title(text);
        f.render_widget(hint, hint_area);
    }
}
