use crate::{App, COLOR_INACTIVE, SymbolSize};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
}; // import your App type

const SELECTED_STRING: &str = "selected";

pub fn render_footer(app: &App, frame: &mut Frame, area: Rect, extra: String) {
    let mode = app.mode.to_string();
    let x = app.current_pos.0;
    let y = app.current_pos.1;
    let pyramid_nbr = app.active_pattern().len();
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2)])
        .split(area);
    let help_paragraph = Paragraph::new(format!(
        "pos:{x},{y}, nbr:{pyramid_nbr}\t Press h to show/hide help menu\t  MODE>{mode}"
    ));
    frame.render_widget(help_paragraph, areas[0]);

    let mut bla = app
        .patterns
        .iter()
        .enumerate()
        .map(|(index, _pat)| {
            if index >= app.render_start_index && index <= app.render_end_index {
                if index == app.selected_pattern_index {
                    format!(" {index} ").red().underlined()
                } else {
                    format!(" {index} ").red()
                }
            } else {
                format!(" {index} ").white()
            }
        })
        .collect::<Vec<Span>>();

    let selected_pattern_index = app.selected_pattern_index;

    bla.push(format!("{extra}").red().underlined());
    bla.push(format!(" ({selected_pattern_index})").underlined());
    let pattern_tracker = Paragraph::new(Line::from(bla));

    frame.render_widget(pattern_tracker, areas[1]);
}

pub fn render_top_down_pyramid(
    frame: &mut Frame,
    area: Rect,
    height: SymbolSize,
    color: Color,
    selected_symbol: bool,
) {
    let mut lines = vec![];

    let pyramid_height = match height {
        SymbolSize::SMALL => 4,
        SymbolSize::MEDIUM => 6,
        SymbolSize::LARGE => 8,
    };

    let empty_line_nbr = match height {
        SymbolSize::SMALL => 2,
        SymbolSize::MEDIUM => 1,
        SymbolSize::LARGE => 0,
    };

    for _ in 0..empty_line_nbr {
        let empty = " ".repeat(1);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(pyramid_height / 2) {
        let count_alt = 1 + i * 2;
        let count = pyramid_height - count_alt;
        let side = "█".repeat(count_alt);
        let side_alt = "▒".repeat(count);

        lines.push(Line::from(Span::styled(
            format!("{side}{side_alt}{side_alt}{side}"),
            Style::default().fg(color),
        )));
    }

    for i in 1..(pyramid_height / 2) {
        let count_alt = pyramid_height - 1 - i * 2;
        let count = pyramid_height - count_alt;
        let side = "█".repeat(count_alt);
        let side_alt = "▒".repeat(count);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{side}{side_alt}{side_alt}{side}"),
            Style::default().fg(color),
        )));
    }

    // Consider writing triangles with these fullheight triangle characters from JuliaFont. Also consider using .bg(color)

    let border_title = if selected_symbol { SELECTED_STRING } else { "" };

    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title_bottom(border_title)
                .title_alignment(Alignment::Center)
                .border_style(if selected_symbol {
                    color
                } else {
                    COLOR_INACTIVE
                }),
        );
    frame.render_widget(paragraph, area);
}

pub fn render_top_down_pyramid_angled(
    frame: &mut Frame,
    area: Rect,
    height: SymbolSize,
    color: Color,
    selected_symbol: bool,
) {
    let mut lines = vec![];
    let pyramid_height = match height {
        SymbolSize::SMALL => 4,
        SymbolSize::MEDIUM => 6,
        SymbolSize::LARGE => 8,
    };

    let empty_line_nbr = match height {
        SymbolSize::SMALL => 2,
        SymbolSize::MEDIUM => 1,
        SymbolSize::LARGE => 0,
    };

    for _ in 0..empty_line_nbr {
        let empty = " ".repeat(1);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(pyramid_height / 2) {
        let count_alt = 1 + i * 2;
        let empty_count = pyramid_height - 1 - i * 2;
        let side = "█".repeat(count_alt);
        let empty = " ".repeat(empty_count);
        let side_alt = "▒".repeat(count_alt);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}{side}{side_alt}{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(pyramid_height / 2) {
        let count_alt = pyramid_height - 1 - i * 2;
        let empty_count = 1 + i * 2;
        let side = "█".repeat(count_alt);
        let empty = " ".repeat(empty_count);
        let side_alt = "▒".repeat(count_alt);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}{side_alt}{side}{empty}"),
            Style::default().fg(color),
        )));
    }

    let border_title = if selected_symbol { SELECTED_STRING } else { "" };

    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title_bottom(border_title)
                .title_alignment(Alignment::Center)
                .border_style(if selected_symbol {
                    color
                } else {
                    COLOR_INACTIVE
                }),
        );
    frame.render_widget(paragraph, area);
}

pub fn render_empty(frame: &mut Frame, area: Rect, color: Color, selected_symbol: bool) {
    let border_title = if selected_symbol { SELECTED_STRING } else { "" };

    let paragraph = Paragraph::new("")
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title_bottom(border_title)
                .title_alignment(Alignment::Center)
                .border_style(if selected_symbol {
                    color
                } else {
                    COLOR_INACTIVE
                }),
        );
    frame.render_widget(paragraph, area);
}
