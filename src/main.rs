use strum_macros::Display;
mod ui;
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
mod hackerman;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
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
    data_big: Vec<Datos>,
}

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
struct Datos {
    pos: (usize, usize),
    pyramid_type: PyramidType,
    color: Color,
    size: SymbolSize,
}

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
            data_big: vec![
                Datos {
                    pos: (0, 0),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightGreen,
                    size: SymbolSize::SMALL,
                },
                Datos {
                    pos: (0, 1),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightYellow,
                    size: SymbolSize::MEDIUM,
                },
                Datos {
                    pos: (0, 2),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightRed,
                    size: SymbolSize::LARGE,
                },
                Datos {
                    pos: (1, 0),
                    pyramid_type: PyramidType::Straight,
                    color: Color::LightRed,
                    size: SymbolSize::SMALL,
                },
            ],
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
        let pyramid_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(layout);
        // render_flat_pyramid(frame, pyramid_layouts[0], 12);
        // render_colored_pyramid(frame, pyramid_layouts[1], 12);
        // render_top_down_pyramid(frame, pyramid_layouts[2], 6);
        // render_top_down_pyramid_angled(frame, pyramid_layouts[1], 6);
        render_footer(self, frame, layout, self.current_pos);
        //
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.area());
        // self.render_saved(frame, layout[0]);

        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(layout[0]);

        for (row_index, layout) in edit_layout.iter().enumerate() {
            let row_layouts = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
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

                        if pyramid.pos == self.current_pos {
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
                            )
                        } else {
                            render_top_down_pyramid(frame, *row_layout, pyramid.size, pyramid_color)
                        }
                    }

                    _ => {
                        let color = if self.current_pos == (col_index, row_index) {
                            Color::Rgb((10), (10), (10))
                        } else {
                            Color::Black
                        };
                        render_top_down_pyramid(frame, *row_layout, SymbolSize::MEDIUM, color)
                    }
                }
            }
        }

        // if self.mode == Mode::EDITING {
        //     let block = Block::bordered().title("Popup");
        //     let area = popup_area(frame.area(), 7);
        //     frame.render_widget(Clear, area); //this clears out the background
        //     self.render_edit(frame, area);
        //     // frame.render_widget(block, area);
        // }

        // if self.show_help {
        //     let help_text =
        //         Paragraph::new("Press h to show/hide help menu\n\nPress e to show/hide adding menu\n\nPress enter to change validity of pattern\n\nPress g to save pattern\n\nPress d to change direction\n\nPress c to change color\n\nPress s to change size").block(Block::bordered().title("Popup"));
        //     let area = popup_area(frame.area(), 30);
        //     frame.render_widget(Clear, area); //this clears out the background
        //     frame.render_widget(help_text, area);
        // };

        // let list = [
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two",
        //     "Three", "Four", "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four",
        //     "Five", "Six", "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        //     "Seven", "Eight", "OneL", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        // ];

        // let (chunks, remainder) = list.as_chunks::<2>();

        // let rows: Vec<Row> = chunks
        //     .iter()
        //     // .map(|(row)| Row::new([Cell::from(row[0]).bg(Color::Cyan)]))
        //     .map(|(chunk)| Row::new(chunk.map(|cell| Cell::from(cell).bg(Color::Cyan))))
        //     .collect();

        // if self.mode == Mode::TESTING {
        //     let widths = [Constraint::Length(20), Constraint::Length(20)];
        //     let block = Table::new(rows, widths)
        //         .cell_highlight_style(self.highlight_style)
        //         .block(
        //             Block::bordered()
        //                 .title_top("g to add")
        //                 .title_bottom("enter to change validity")
        //                 .border_type(BorderType::Rounded),
        //         )
        //         .flex(Flex::Center);
        //     frame.render_widget(Clear, frame.area()); //this clears out the background
        //     frame.render_widget(block, frame.area());
        // }

        self.render_footer(frame, layout[1]);
    }

    fn render_edit(&mut self, frame: &mut Frame, layout: Rect) {
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let cells = data.iter().map(|data| {
                Cell::from(Text::from(data.name.to_string()).style(Style::new().fg(data.color)))
            });
            return Row::new(cells);
        });

        let widths = [
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ];

        let border_color = self.get_border_color();
        let table = Table::new(rows, widths)
            .cell_highlight_style(self.highlight_style)
            .block(
                Block::bordered()
                    .title_top("g to add")
                    .title_bottom("enter to change validity")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(border_color)),
            )
            .flex(Flex::Center);

        frame.render_stateful_widget(table, layout, &mut self.state);
    }

    fn render_saved(&self, frame: &mut Frame, layout: Rect) {
        fn get_border_color_2(correct: bool) -> Color {
            match correct {
                true => Color::LightGreen,
                false => Color::LightRed,
            }
        }

        let row_length = 5;
        let nbr_of_rows = self.saved_items.len().div_ceil(row_length);
        let constraints: Vec<Constraint> =
            (0..nbr_of_rows).map(|_| Constraint::Length(7)).collect();
        let widths = [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ];

        let saved_pattern_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(layout);

        for (pattern_index, pattern) in self.saved_items.iter().enumerate() {
            let is_correct = self.saved_correct[pattern_index];
            let rows_2 = pattern.iter().enumerate().map(|(i, data)| {
                let cells = data.iter().map(|data| {
                    Cell::from(Text::from(data.name.to_string()).style(Style::new().fg(data.color)))
                });
                return Row::new(cells);
            });
            let table_2 = Table::new(rows_2, widths)
                .cell_highlight_style(self.highlight_style)
                .block(
                    Block::bordered()
                        .title_top(if is_correct { "Valid" } else { "Invalid" })
                        .title_alignment(Alignment::Center)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(get_border_color_2(is_correct))),
                )
                .flex(Flex::Center);

            let row_index = (pattern_index) / row_length;
            let col_index = pattern_index % row_length;

            let column_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints((0..row_length).map(|_| Constraint::Length(14)))
                .split(saved_pattern_layout[row_index]);
            frame.render_widget(table_2, column_layout[col_index]);
        }
    }

    fn get_border_color(&self) -> Color {
        let border_color = if self.correct {
            Color::LightGreen
        } else {
            Color::LightRed
        };
        border_color
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Right) => {
                if self.current_pos.0 > 4 {
                    self.current_pos.0 = 0
                } else {
                    self.current_pos.0 = self.current_pos.0 + 1
                }
            }
            (_, KeyCode::Left) => {
                if self.current_pos.0 == 0 {
                    self.current_pos.0 = 4
                } else {
                    self.current_pos.0 = self.current_pos.0 - 1
                }
            }

            (_, KeyCode::Down) => {
                if self.current_pos.1 > 4 {
                    self.current_pos.1 = 0
                } else {
                    self.current_pos.1 = self.current_pos.1 + 1
                }
            }
            (_, KeyCode::Up) => {
                if self.current_pos.1 == 0 {
                    self.current_pos.1 = 4
                } else {
                    self.current_pos.1 = self.current_pos.1 - 1
                }
            }
            (_, KeyCode::Enter) => {
                let object = self
                    .data_big
                    .iter()
                    .find(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                // if let Some(&mut object) = object {
                //     object.pyramid_type = PyramidType::Angled;
                // }
                //
                //

                let mut new_type = PyramidType::Straight;
                let mut old_color = Color::LightGreen;
                let mut old_size = SymbolSize::MEDIUM;
                if object.is_some() {
                    old_color = object.unwrap().color;
                    old_size = object.unwrap().size;
                    let index_of_selected = self.data_big.iter().position(|d| {
                        d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1
                    });

                    new_type = match object.unwrap().pyramid_type {
                        PyramidType::Straight => PyramidType::Angled,
                        _ => PyramidType::Straight,
                    };

                    self.data_big.remove(index_of_selected.unwrap());
                }

                self.data_big.push(Datos {
                    pyramid_type: new_type,
                    pos: self.current_pos,
                    color: old_color,
                    size: old_size,
                });
            }
            (_, KeyCode::Char('c')) => {
                let object = self
                    .data_big
                    .iter()
                    .find(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                if object.is_none() {
                    return;
                }
                let index_of_selected = self
                    .data_big
                    .iter()
                    .position(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                let new_color = match object.unwrap().color {
                    Color::LightRed => Color::LightGreen,
                    Color::LightGreen => Color::LightYellow,
                    Color::LightYellow => Color::LightRed,
                    _ => Color::LightRed,
                };
                let old_type = object.unwrap().pyramid_type;
                let old_size = object.unwrap().size;
                self.data_big.remove(index_of_selected.unwrap());

                self.data_big.push(Datos {
                    pyramid_type: old_type,
                    pos: self.current_pos,
                    color: new_color,
                    size: old_size,
                });
            }
            (_, KeyCode::Backspace) => {
                let object = self
                    .data_big
                    .iter()
                    .find(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                if object.is_none() {
                    return;
                }
                let index_of_selected = self
                    .data_big
                    .iter()
                    .position(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                self.data_big.remove(index_of_selected.unwrap());
            }

            (_, KeyCode::Char('s')) => {
                let object = self
                    .data_big
                    .iter()
                    .find(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                if object.is_none() {
                    return;
                }
                let index_of_selected = self
                    .data_big
                    .iter()
                    .position(|d| d.pos.0 == self.current_pos.0 && d.pos.1 == self.current_pos.1);

                let new_size = match object.unwrap().size {
                    SymbolSize::SMALL => SymbolSize::MEDIUM,
                    SymbolSize::MEDIUM => SymbolSize::LARGE,
                    SymbolSize::LARGE => SymbolSize::SMALL,
                    _ => SymbolSize::MEDIUM,
                };
                let old_type = object.unwrap().pyramid_type;
                let old_color = object.unwrap().color;
                self.data_big.remove(index_of_selected.unwrap());

                self.data_big.push(Datos {
                    pyramid_type: old_type,
                    pos: self.current_pos,
                    color: old_color,
                    size: new_size,
                });
            }
            _ => {}
        }
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event_old(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Char('d')) => self.change_char(),
            (_, KeyCode::Char('s')) => self.change_size(),
            (_, KeyCode::Char('x')) => self.remove_char(),
            (_, KeyCode::Char('c')) => self.change_color(),
            (_, KeyCode::Char('g')) => self.save_pattern(),
            (_, KeyCode::Char('h')) => self.show_help = !self.show_help,
            (_, KeyCode::Enter) => self.set_correct(),
            (_, KeyCode::Backspace) => self.clear_history(),

            (_, KeyCode::Char('e')) => {
                let new_mode = match (self.mode) {
                    Mode::TESTING => Mode::HISTORY,
                    Mode::EDITING => Mode::VIEWING,
                    Mode::VIEWING => Mode::TESTING,
                    _ => Mode::EDITING,
                };

                self.mode = new_mode;
            }
            (_, KeyCode::Char('v')) => self.mode = Mode::VIEWING,

            (_, KeyCode::Char('l') | KeyCode::Right) => self.next_column(),
            (_, KeyCode::Char('h') | KeyCode::Left) => self.previous_column(),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.next_row(),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.previous_row(),
            _ => {}
        }
    }

    pub fn clear_history(&mut self) {
        self.saved_items.remove(2);
    }
    pub fn change_direction(&mut self) {
        let (x, y) = self.get_pos();
        let new_direction = match self.items[x][y].direction {
            SymbolDirection::UP => SymbolDirection::RIGHT,
            SymbolDirection::RIGHT => SymbolDirection::DOWN,
            SymbolDirection::DOWN => SymbolDirection::LEFT,
            SymbolDirection::LEFT => SymbolDirection::UP,
            _ => SymbolDirection::UP,
        };

        self.items[x][y].direction = new_direction;
    }

    pub fn remove_char(&mut self) {
        let (x, y) = self.get_pos();
        self.items[x][y].name = ' ';
    }

    pub fn set_correct(&mut self) {
        let (x, y) = self.get_pos();
        self.correct = !self.correct
    }

    pub fn save_pattern(&mut self) {
        let copied_items = self.items.clone();
        self.saved_items.push(copied_items);
        self.saved_correct.push(self.correct);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }

            None => 0,
        };

        self.state.select(Some(i));

        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }

            None => 0,
        };

        self.state.select(Some(i));

        // self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }
    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    fn change_size(&mut self) {
        let (x, y) = self.get_pos();

        let current_char = self.items[x][y].name;
        let next_char = match current_char {
            '⭡' => '⇑',
            '⇑' => '⤊',
            '⤊' => '⭡',

            '⭢' => '⇒',
            '⇒' => '⇛',
            '⇛' => '⭢',

            '⭣' => '⇓',
            '⇓' => '⤋',
            '⤋' => '⭣',

            '⭠' => '⇐',
            '⇐' => '⇚',
            '⇚' => '⭠',

            _ => '⭡',
        };

        self.items[x][y].name = next_char;
    }

    fn change_char(&mut self) {
        let (x, y) = self.get_pos();

        let current_char = self.items[x][y].name;
        let next_char = match current_char {
            '⭡' => '⭢',
            '⭢' => '⭣',
            '⭣' => '⭠',
            '⭠' => '⭡',

            '⇑' => '⇒',
            '⇒' => '⇓',
            '⇓' => '⇐',
            '⇐' => '⇑',

            '⤊' => '⇛',
            '⇛' => '⤋',
            '⤋' => '⇚',
            '⇚' => '⤊',

            _ => '⭡',
        };

        self.items[x][y].name = next_char;
    }

    fn get_pos(&mut self) -> (usize, usize) {
        let selected_col = self.state.selected_cell();
        let (x, y) = match selected_col {
            Some(d) => (d.0, d.1),
            None => (0, 0),
        };
        (x, y)
    }

    fn change_color(&mut self) {
        let (x, y) = self.get_pos();

        let current_color = self.items[x][y].color;
        let next_color = match current_color {
            Color::LightRed => Color::Blue,
            Color::Blue => Color::White,
            Color::White => Color::LightRed,
            _ => Color::LightRed,
        };

        self.items[x][y].color = next_color;
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
