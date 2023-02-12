use crate::ui_app;
use crossterm::event::{KeyCode, KeyEvent};
use rusted_wizard_core::{Wizard, WizardState};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;
use Vec;
use tui::style::Color::{Black, White};

pub fn handle_input(app: &mut ui_app::App, key: KeyEvent) {}

pub fn draw<'a, B: Backend>(f: &mut Frame<B>, app: &'a ui_app::App) {
    if app.game.is_none() {
        return;
    }

    let mut game = app.game.as_ref().unwrap();
    if game.state == WizardState::Init || game.state == WizardState::End { return; }

    // background
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Black));
    f.render_widget(block, f.size());
    let mut playground_area = f.size();
    playground_area.x += 3;
    playground_area.y += 1;
    playground_area.height -= 2;
    playground_area.width -= 4;
    let multiplier = if game.round_count * 2 + 3 < playground_area.height as usize { 2 } else { 1 };
    let upcoming_row_offset = if multiplier == 1 { 1 } else { 0 };

    let column_width_percentage = 100 / game.player_count;
    let mut column_constraints: Vec<Constraint> = vec![];
    for _ in 0..game.player_count {
        column_constraints.push(Constraint::Percentage(column_width_percentage as u16));
    }

    let column_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(column_constraints.as_ref())
        .split(playground_area);

    for round_index in 0..game.round_count {
        let str = (round_index + 1).to_string();
        let round_index_paragraph = Paragraph::new(str)
            .style(Style::default().add_modifier(if round_index == game.round_index {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }));
        let y = 4 + (round_index * multiplier) + (if round_index > game.round_index { upcoming_row_offset } else { 0 });

        let round_index_area = Rect::new(1, y as u16, 2, 1);
        f.render_widget(round_index_paragraph, round_index_area);
    }

    for i in 0..game.player_count {
        let player_area = column_layout[i];
        let block = Block::default()
            .borders(Borders::LEFT);
        f.render_widget(block, player_area);

        let player_name = app.player_names[i].clone(); // players of game will be rotated!
        let name_paragraph = Paragraph::new(player_name)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(Style::default().add_modifier(Modifier::BOLD).fg(White));
        let mut player_name_area = player_area;
        player_name_area.height = 2;
        player_name_area.width -= 3;
        player_name_area.x += 2;
        f.render_widget(name_paragraph, player_name_area);

        for round_draw_index in 0..game.round_count {
            let mut round_points_area = player_area;
            round_points_area.x += 2;
            round_points_area.y += 3 + (round_draw_index * multiplier) as u16;
            round_points_area.height = 1;
            round_points_area.width -= 9;

            if round_draw_index > game.round_index {
                round_points_area.y += upcoming_row_offset as u16;
            }

            let round_points_str = if round_draw_index < game.round_index {
                game.rounds[round_draw_index].players[i].points.to_string()
            } else {
                String::new()
            };
            let round_points_paragraph = Paragraph::new(round_points_str)
                .style(Style::default().bg(Color::Gray));
            f.render_widget(round_points_paragraph, round_points_area);

            let round_tip_area = Rect::new(round_points_area.x + round_points_area.width + 1, round_points_area.y, 2, 1);
            let round_tip_paragraph = Paragraph::new(String::new())
                .style(Style::default().bg(Color::Gray));
            f.render_widget(round_tip_paragraph, round_tip_area);

            let round_match_area = Rect::new(round_tip_area.x + round_tip_area.width + 1, round_points_area.y, 2, 1);
            let round_match_paragraph = Paragraph::new(String::new())
                .style(Style::default().bg(Color::Gray));
            f.render_widget(round_match_paragraph, round_match_area);
        }
    }
}