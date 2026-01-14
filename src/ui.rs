use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame, Line, Span, Style},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{app::App, game::Player};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(f.size());

    let board_text = board_as_lines(app);

    let board = Paragraph::new(board_text)
        .block(Block::default().title("Gomoku").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(board, chunks[0]);

    let status = status_text(app);
    let status = Paragraph::new(status)
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(status, chunks[1]);
}

fn board_as_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::with_capacity(app.game.size());

    for r in 0..app.game.size() {
        let mut spans: Vec<Span<'static>> = Vec::with_capacity(app.game.size() * 2);
        for c in 0..app.game.size() {
            let ch = match app.game.board()[r][c] {
                Some(p) => p.symbol(),
                None => '.',
            };

            let mut style = Style::default();
            if r == app.cursor_row && c == app.cursor_col {
                style = style.reversed();
            }

            spans.push(Span::styled(ch.to_string(), style));
            spans.push(Span::raw(" "));
        }
        lines.push(Line::from(spans));
    }

    lines
}

fn status_text(app: &App) -> String {
    let turn = match app.current_player() {
        Player::Black => "Black (X)",
        Player::White => "White (O)",
    };

    let winner = app
        .game
        .winner()
        .map(|p| match p {
            Player::Black => "Black (X)",
            Player::White => "White (O)",
        })
        .unwrap_or("None");

    format!(
        "Turn: {turn}\nWinner: {winner}\nControls: arrows/hjkl move, Enter/Space place, r restart, q quit"
    )
}
