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
    data_big: Pattern,
    patterns: Vec<Pattern>,
    pattern_index: usize,
}

#[derive(Debug, Clone)]
pub struct Pattern(pub Vec<Datos>);

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
            data_big: Pattern(vec![
                Datos {
                    pos: (0, 0),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::SMALL,
                },
                Datos {
                    pos: (1, 0),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::MEDIUM,
                },
                Datos {
                    pos: (2, 0),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::LARGE,
                },
                Datos {
                    pos: (0, 1),
                    pyramid_type: PyramidType::Angled,
                    color: Color::LightGreen,
                    size: SymbolSize::SMALL,
                },
                Datos {
                    pos: (1, 1),
                    pyramid_type: PyramidType::Angled,
                    color: Color::LightGreen,
                    size: SymbolSize::MEDIUM,
                },
                Datos {
                    pos: (2, 1),
                    pyramid_type: PyramidType::Angled,
                    color: Color::LightGreen,
                    size: SymbolSize::LARGE,
                },
                Datos {
                    pos: (0, 2),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::LARGE,
                },
                Datos {
                    pos: (1, 2),
                    pyramid_type: PyramidType::Angled,
                    color: Color::LightGreen,
                    size: SymbolSize::LARGE,
                },
                Datos {
                    pos: (2, 2),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::LARGE,
                },
            ]),
            patterns: vec![],
            pattern_index: 0,
        }
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

    fn render_footer(&mut self, frame: &mut Frame, layout: Rect) {
        render_footer(self, frame, layout, self.current_pos);
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(2)])
            .split(frame.area());
        // self.render_saved(frame, layout[0]);
        //

        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints((0..HEIGHT).map(|_| Constraint::Length(16)))
            .split(layout[0]);

        for (row_index, layout) in edit_layout.iter().enumerate() {
            let row_layouts = Layout::default()
                .direction(Direction::Horizontal)
                .constraints((0..WIDTH).map(|_| Constraint::Length(20)))
                .split(*layout);

            for (col_index, row_layout) in row_layouts.iter().enumerate() {
                let pyramid = self
                    .data_big
                    .iter()
                    .find(|d| d.pos.0 == col_index && d.pos.1 == row_index);
                match pyramid {
                    Some(pyramid) => {
                        let mut pyramid_color = match pyramid.pos {
                            _ => pyramid.color,
                        };

                        let selected_symbol = pyramid.pos == self.current_pos;

                        if selected_symbol {
                            pyramid_color = match pyramid.color {
                                Color::LightGreen => Color::Green,
                                Color::LightRed => Color::Red,
                                Color::LightYellow => Color::Yellow,
                                _ => Color::DarkGray,
                            }
                        }

                        if pyramid.pyramid_type == PyramidType::Angled {
                            render_top_down_pyramid_angled(
                                frame,
                                *row_layout,
                                pyramid.size,
                                pyramid_color,
                                selected_symbol,
                            )
                        } else {
                            render_top_down_pyramid(
                                frame,
                                *row_layout,
                                pyramid.size,
                                pyramid_color,
                                selected_symbol,
                            )
                        }
                    }

                    _ => {
                        let color = if self.current_pos == (col_index, row_index) {
                            Color::Rgb((10), (10), (10))
                        } else {
                            Color::Black
                        };
                        render_top_down_pyramid(
                            frame,
                            *row_layout,
                            SymbolSize::MEDIUM,
                            color,
                            false,
                        )
                    }
                }
            }
        }

        self.render_footer(frame, layout[1]);
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
