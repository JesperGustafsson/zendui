use strum_macros::Display;
mod helpers;
mod ui;
use crate::helpers::key_handler::{self, *};
use crate::ui::footer::*;
use color_eyre::{Result, owo_colors::OwoColorize};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect, Rows},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Cell, Clear, Paragraph, Row, Table, TableState, Widget},
};
use std::ops::{Deref, DerefMut};

mod hackerman;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

impl Deref for Pattern {
    type Target = Vec<Datos>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    state: TableState,
    items: Vec<Vec<Data>>,
    highlight_style: Style,
    saved_items: Vec<Vec<Vec<Data>>>,
    saved_correct: Vec<bool>,
    correct: bool,
    mode: Mode,
    box_height: u16,
    show_help: bool,
    current_pos: (usize, usize),
    data_big: PatternParent,
    patterns: Vec<PatternParent>,
    pattern_index: usize,
    render_start_index: usize,
    render_end_index: usize,
    patterns_per_row: usize,
    pattern_rows: usize,
    selected_pattern_index: usize,
}

#[derive(Debug, Clone)]
pub struct Pattern(pub Vec<Datos>);

#[derive(Debug, Clone)]
pub struct PatternParent {
    data: Pattern,
    valid: bool,
}

// #[derive(Debug, Clone)]
// pub struct Pattern {
//     data: Vec<Datos>,
//     valid: bool,
// }

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum PyramidType {
    Straight,
    Angled,
}

#[derive(Debug, Clone)]
enum SymbolDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
enum Mode {
    ADDING,
    VIEWING,
    CHOSING,
    EDITING,
    TESTING,
    HISTORY,
}

#[derive(Debug, Clone, Copy)]
enum SymbolSize {
    SMALL,
    MEDIUM,
    LARGE,
}

#[derive(Debug, Clone)]
struct Data {
    name: char,
    color: Color,
    direction: SymbolDirection,
    size: SymbolSize,
}

fn gen_default_cell() -> Data {
    return Data {
        name: ' ',
        color: Color::Black,
        direction: SymbolDirection::UP,
        size: SymbolSize::SMALL,
    };
}

fn popup_area(area: Rect, length: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(length)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(length * 2)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

#[derive(Debug, Clone)]
pub struct Datos {
    pos: (usize, usize),
    pyramid_type: PyramidType,
    color: Color,
    size: SymbolSize,
}

const HEIGHT: usize = 3;
const WIDTH: usize = 3;

// const COLOR_INACTIVE: Color = Color::DarkGray;
// const COLOR_RED: Color = Color::Red;
// const COLOR_GREEN: Color = Color::Green;
// const COLOR_YELLOW: Color = Color::Yellow;
// const COLOR_RED_LIGHT: Color = Color::LightRed;
// const COLOR_GREEN_LIGHT: Color = Color::LightGreen;
// const COLOR_YELLOW_LIGHT: Color = Color::LightYellow;

const COLOR_INACTIVE: Color = Color::Rgb(74, 74, 74);

const COLOR_RED: Color = Color::Rgb(230, 124, 124);
const COLOR_RED_ACTIVE: Color = Color::Rgb(186, 20, 20);

const COLOR_BLUE: Color = Color::Rgb(84, 96, 222);
const COLOR_BLUE_ACTIVE: Color = Color::Rgb(35, 47, 173);

const COLOR_YELLOW: Color = Color::Rgb(235, 211, 117);
const COLOR_YELLOW_ACTIVE: Color = Color::Rgb(245, 200, 24);

const PATTERN_BORDER_VALID: Color = Color::Rgb(55, 150, 55);
const PATTERN_BORDER_VALID_ACTIVE: Color = Color::Rgb(0, 255, 0);
const PATTERN_BORDER_INVALID: Color = Color::Rgb(150, 55, 55);
const PATTERN_BORDER_INVALID_ACTIVE: Color = Color::Rgb(255, 0, 0);

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let highlight_style = Style::new()
            .italic()
            .bold()
            .bg(Color::Rgb((20), (20), (20)));

        let num_items = 4;

        let data_vec: Vec<Vec<Data>> = (0..num_items)
            .map(|_| (0..num_items).map(|_| gen_default_cell()).collect())
            .collect();

        Self {
            running: true,
            state: TableState::default().with_selected_cell((0, 0)),
            items: data_vec,
            highlight_style: highlight_style,
            saved_items: vec![],
            correct: false,
            saved_correct: vec![],
            mode: Mode::VIEWING,
            box_height: 7,
            show_help: false,
            current_pos: (0, 0),
            data_big: PatternParent {
                data: Pattern(vec![]),
                valid: true,
            },
            render_start_index: 0,
            render_end_index: 0,
            pattern_rows: 2,
            patterns_per_row: 2,
            patterns: vec![],
            pattern_index: 0,
            selected_pattern_index: 0,
        }
    }

    pub fn active_pattern(&self) -> &Pattern {
        &self.data_big.data
    }

    pub fn active_viewed_pattern(&self) -> &Pattern {
        let pattern = self.patterns.get(self.selected_pattern_index).unwrap();
        let actual_pattern = &pattern.data;
        return actual_pattern;
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render_footer(&mut self, frame: &mut Frame, layout: Rect, extra: String) {
        render_footer(self, frame, layout, self.current_pos, extra);
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(2)])
            .split(frame.area());
        // self.render_saved(frame, layout[0]);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Length(10)])
            .split(layout[0]);
        let saved_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..self.pattern_rows).map(|_| Constraint::Fill(1)))
            .split(inner_layout[0]);

        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..HEIGHT).map(|_| Constraint::Length(10)))
            .split(saved_layouts[0]);

        // self.render_pattern(
        //     frame,
        //     saved_layouts[0],
        //     self.data_big.clone(),
        //     "0".to_string(),
        // );
        //
        //
        //
        //

        let render_end_index2 = (self.render_end_index + 1).min(self.patterns.len());
        let patterns_to_render =
            self.patterns.clone()[self.render_start_index..render_end_index2].to_vec();

        for (index, saved_pattern) in patterns_to_render.iter().enumerate() {
            // let actual_index = self.patterns.len() - 1 - index;
            let actual_index = index;
            let layout_row_index = actual_index / (self.patterns_per_row);
            let layout_col_index = actual_index % (self.patterns_per_row);
            let sub_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints((0..self.patterns_per_row).map(|_| Constraint::Fill(1)))
                .split(saved_layouts[layout_row_index]);

            let extr = format!("{layout_row_index},{layout_col_index}[{actual_index}]");
            self.render_pattern(
                frame,
                sub_layout[layout_col_index],
                saved_pattern.clone(),
                extr,
                index,
            );
            // self.render_pattern(frame, &prev_layout, &prev_pattern.unwrap().clone());
        }

        self.render_footer(frame, layout[1], "".to_string());

        if self.mode == Mode::EDITING {
            let a = popup_area(frame.area(), 34);
            frame.render_widget(Clear, a);

            self.render_pattern(frame, a, self.data_big.clone(), "0".to_string(), 0);
        }
    }

    fn render_pattern(
        &mut self,
        frame: &mut Frame<'_>,
        edit_layout: Rect,
        pattern_parent: PatternParent,
        extra: String,
        pattern_index: usize,
    ) {
        let pattern = pattern_parent.data;
        let subbo_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)])
            .split(edit_layout);

        let pattern_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..HEIGHT).map(|_| Constraint::Length(10)))
            .margin(1)
            .split(subbo_layout[0]);

        // self.render_footer(frame, subbo_layout[1], extra);
        //

        let border_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(60 + 2)])
            .split(edit_layout);

        let mut border_color = if pattern_parent.valid {
            PATTERN_BORDER_VALID
        } else {
            PATTERN_BORDER_INVALID
        };

        let global_pattern_index = self.render_start_index + pattern_index;
        let is_selected = global_pattern_index == self.selected_pattern_index;

        if (is_selected) {
            match border_color {
                PATTERN_BORDER_VALID => border_color = PATTERN_BORDER_VALID_ACTIVE,
                PATTERN_BORDER_INVALID => border_color = PATTERN_BORDER_INVALID_ACTIVE,
                _ => (),
            }
        };

        let block_widget = Block::bordered()
            .border_style(border_color)
            .title(format!("#{global_pattern_index}"));

        frame.render_widget(block_widget, border_layout[0]);

        for (row_index, layout) in pattern_layout.iter().enumerate() {
            let row_layouts = Layout::default()
                .direction(Direction::Horizontal)
                .constraints((0..WIDTH).map(|_| Constraint::Length(20)))
                .split(*layout);

            for (col_index, row_layout) in row_layouts.iter().enumerate() {
                let pyramid = pattern
                    .iter()
                    .find(|d| d.pos.0 == col_index && d.pos.1 == row_index);
                let selected_symbol = self.current_pos == (col_index, row_index) && is_selected;
                match pyramid {
                    Some(pyramid) => {
                        let mut pyramid_color = match pyramid.pos {
                            _ => pyramid.color,
                        };

                        // let selected_symbol = pyramid.pos == self.current_pos;

                        if selected_symbol {
                            pyramid_color = match pyramid.color {
                                COLOR_BLUE => COLOR_BLUE_ACTIVE,
                                COLOR_RED => COLOR_RED_ACTIVE,
                                COLOR_YELLOW => COLOR_YELLOW_ACTIVE,
                                _ => COLOR_INACTIVE,
                            }
                        }

                        if pyramid.pyramid_type == PyramidType::Angled {
                            render_top_down_pyramid_angled(
                                frame,
                                *row_layout,
                                pyramid.size,
                                pyramid_color,
                                selected_symbol,
                                self,
                            )
                        } else {
                            render_top_down_pyramid(
                                frame,
                                *row_layout,
                                pyramid.size,
                                pyramid_color,
                                selected_symbol,
                                self,
                            )
                        }
                    }

                    _ => render_empty(
                        frame,
                        *row_layout,
                        SymbolSize::MEDIUM,
                        Color::White,
                        selected_symbol,
                    ),
                }
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => on_key_event(self, key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
