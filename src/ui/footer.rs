use crate::{App, SymbolSize};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
}; // import your App type

pub fn render_footer(app: &App, frame: &mut Frame, area: Rect, pos: (usize, usize)) {
    let mode = app.mode.to_string();
    let x = app.current_pos.0;
    let y = app.current_pos.1;
    let pyramid_nbr = app.data_big.len();
    frame.render_widget(
        Paragraph::new(format!(
            "pos:{x},{y}, nbr:{pyramid_nbr}\t Press h to show/hide help menu\t  MODE>{mode}"
        )),
        area,
    );
}

pub fn render_colored_pyramid(frame: &mut Frame, area: Rect, height: usize) {
    let mut lines = vec![];

    for i in 0..height {
        let spaces = " ".repeat(height - i - 1);
        let left_side = Span::styled("█".repeat(i + 1), Style::default().fg(Color::Blue));
        let right_side = Span::styled("█".repeat(i), Style::default().fg(Color::Yellow));

        let line = Line::from(vec![Span::raw(spaces), left_side, right_side]);

        lines.push(line);
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

pub fn render_side_pyramid(frame: &mut Frame, area: Rect, height: usize) {
    let mut lines = vec![];

    // Ascending part (left side, blue)
    for i in 1..=height {
        lines.push(Line::from(Span::styled(
            "█".repeat(i),
            Style::default().fg(Color::Blue),
        )));
    }

    // Descending part (right side, yellow)
    for i in (1..height).rev() {
        lines.push(Line::from(Span::styled(
            "█".repeat(i),
            Style::default().fg(Color::Yellow),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

pub fn render_flat_pyramid(frame: &mut Frame, area: Rect, height: usize) {
    let mut lines = vec![];

    for i in 0..height {
        let count = i * 2 + 1;
        let color = if i < height / 2 {
            Color::Blue
        } else {
            Color::Yellow
        };
        lines.push(Line::from(Span::styled(
            "█".repeat(count),
            Style::default().fg(color),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

pub fn render_top_down_pyramid(frame: &mut Frame, area: Rect, height: SymbolSize, color: Color) {
    let mut lines = vec![];

    let height = match height {
        SymbolSize::SMALL => 8,
        SymbolSize::MEDIUM => 12,
        SymbolSize::LARGE => 16,
        _ => 12,
    };

    for i in 0..((16 - height) / 2) {
        let empty = " ".repeat(1);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(height / 2) {
        let count_alt = 1 + i * 2;
        let count = height - count_alt;
        let side = "█".repeat(count_alt);
        let side_alt = "▒".repeat(count);

        lines.push(Line::from(Span::styled(
            format!("{side}{side_alt}{side_alt}{side}"),
            Style::default().fg(color),
        )));
    }

    for i in 1..(height / 2) {
        let count_alt = height - 1 - i * 2;
        let count = height - count_alt;
        let side = "█".repeat(count_alt);
        let side_alt = "▒".repeat(count);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{side}{side_alt}{side_alt}{side}"),
            Style::default().fg(color),
        )));
    }

    let paragraph = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}

pub fn render_top_down_pyramid_angled(
    frame: &mut Frame,
    area: Rect,
    height: SymbolSize,
    color: Color,
) {
    let mut lines = vec![];
    let height = match height {
        SymbolSize::SMALL => 8,
        SymbolSize::MEDIUM => 12,
        SymbolSize::LARGE => 16,
        _ => 12,
    };

    for i in 0..((16 - height) / 2) {
        let empty = " ".repeat(1);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(height / 2) {
        let count_alt = 1 + i * 2;
        let empty_count = height - 1 - i * 2;
        let count = height - empty_count;
        let side = "█".repeat(count_alt);
        let empty = " ".repeat(empty_count);
        let side_alt = "▒".repeat(count_alt);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}{side}{side_alt}{empty}"),
            Style::default().fg(color),
        )));
    }

    for i in 0..(height / 2) {
        let count_alt = height - 1 - i * 2;
        let empty_count = 1 + i * 2;
        let count = height - empty_count;
        let side = "█".repeat(count_alt);
        let empty = " ".repeat(empty_count);
        let side_alt = "▒".repeat(count_alt);

        lines.push(Line::from(Span::styled(
            // "█".repeat(count_alt).push_str("▒".repeat(2)),
            format!("{empty}{side_alt}{side}{empty}"),
            Style::default().fg(color),
        )));
    }

    let paragraph = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}
