use crate::ui_app;
use crossterm::event::{KeyCode, KeyEvent};
use rusted_wizard_core::Wizard;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn handle_input(app: &mut ui_app::App, key: KeyEvent) {
    if app.game.is_some() { return; }

    if app.player_count.is_empty() {
        app.hint = String::from("Number of players required");
    } else {
        app.hint = String::new();
    }

    match key.code {
        KeyCode::Enter => match app.player_count.parse::<usize>() {
            Ok(player_count) => {
                app.game = Option::from(Wizard::new(player_count));
                for _ in 0..player_count {
                    app.player_names.push(String::new());
                }
            }
            Err(_) => app.hint = String::from("No valid input!"),
        },
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
        _ => {}
    }
}

pub fn draw<'a, B: Backend>(f: &mut Frame<B>, app: &'a ui_app::App) {
    if app.game.is_none() {
        // we only need to render if no game is initialized
        let size = f.size();
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Percentage(50),
                ]
                    .as_ref(),
            )
            .split(size);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ]
                    .as_ref(),
            );

        let text_area = horizontal_layout.split(popup_layout[1])[1];
        let hint_area = horizontal_layout.split(popup_layout[2])[1];

        f.render_widget(Clear, text_area); //this clears out the background
        f.render_widget(Clear, hint_area); //this clears out the background

        let input = Paragraph::new(app.player_count.clone())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("How many Players?"),
            );
        f.render_widget(input, text_area);
        f.set_cursor(
            text_area.x + app.player_count.width() as u16 + 1,
            text_area.y + 1,
        );

        if !app.hint.is_empty() {
            let text = vec![
                Span::from(Span::styled(
                    "Hint: ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Span::from(app.hint.clone()),
            ];

            let hint = Block::default().title(text);
            f.render_widget(hint, hint_area);
        }
    }
}
