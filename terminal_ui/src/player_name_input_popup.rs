use crate::ui_app;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusted_wizard_core::WizardState;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color::{Black, Gray, White};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn handle_input(app: &mut ui_app::App, key: KeyEvent) {
    if app.game.is_some() {
        let mut wizard = app.game.as_mut().unwrap();

        if wizard.state != WizardState::Init {
            return;
        }

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
                        wizard.play(
                            |player_index| app.player_names[player_index].clone(),
                            |_, _| 0,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn draw<'a, B: Backend>(f: &mut Frame<B>, app: &'a ui_app::App) {
    if app.game.is_some() {
        let mut game = app.game.as_ref().unwrap();

        if game.state == WizardState::Init {
            let mut vertical_constraints = vec![Constraint::Percentage(40)];
            for _ in 0..game.player_count {
                vertical_constraints.push(Constraint::Length(1));
            }
            vertical_constraints.push(Constraint::Percentage(40));

            let horizontal_constraints = vec![
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ];

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vertical_constraints.as_ref());
            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(horizontal_constraints)
                .split(f.size());

            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray));

            let mut bg_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    vec![
                        Constraint::Percentage(40),
                        Constraint::Length(game.player_count.clone() as u16),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(horizontal_layout[1])[1];
            bg_area.height += 2;
            bg_area.width += 2;
            bg_area.x -= 1;
            bg_area.y -= 1;
            f.render_widget(block, bg_area);

            for player_index in 0..game.player_count {
                let name = &app.player_names[player_index];

                let style = if player_index == app.player_name_index {
                    Style::default().bg(Gray).fg(Black)
                } else {
                    Style::default().fg(White)
                };

                let paragraph = Paragraph::new(name.clone()).style(style);
                let mut area = vertical_layout.split(horizontal_layout[1])[player_index + 1];
                let mut index_area = area.clone();
                index_area.width = 2;
                area.x += 3;
                area.width -= 3;

                let index_string = (player_index + 1).to_string() + ":";
                let index_paragraph = Paragraph::new(index_string).style(style);

                f.render_widget(index_paragraph, index_area);
                f.render_widget(paragraph, area);
            }

            let area = vertical_layout.split(horizontal_layout[1])[app.player_name_index + 1];
            let x_offset = app.player_names[app.player_name_index].width() as u16;
            f.set_cursor(area.x + x_offset + 3, area.y);
        }
    }
}
