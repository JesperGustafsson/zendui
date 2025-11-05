use std::rc::Rc;
use std::str::Chars;

use color_eyre::{Result, owo_colors::OwoColorize};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Cell, Clear, Paragraph, Row, Table, TableState},
};

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
}

#[derive(Debug, Clone)]
enum SymbolDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mode {
    ADDING,
    VIEWING,
    CHOSING,
    EDITING,
}

#[derive(Debug, Clone)]
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
            mode: Mode::EDITING,
            box_height: 7,
            show_help: true,
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

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.area());
        self.render_saved(frame, layout[0]);

        if self.mode == Mode::EDITING {
            let block = Block::bordered().title("Popup");
            let area = popup_area(frame.area(), 7);
            frame.render_widget(Clear, area); //this clears out the background
            self.render_edit(frame, area);
            // frame.render_widget(block, area);
        }

        if self.show_help {
            let help_text =
                Paragraph::new("Press h to show/hide help menu\n\nPress e to show/hide adding menu\n\nPress enter to change validity of pattern\n\nPress g to save pattern\n\nPress d to change direction\n\nPress c to change color\n\nPress s to change size").block(Block::bordered().title("Popup"));
            let area = popup_area(frame.area(), 30);
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(help_text, area);
        };

        self.render_footer(frame, layout[1]);
    }

    fn render_footer(&mut self, frame: &mut Frame, layout: Rect) {
        frame.render_widget(Paragraph::new("Press h to show/hide help menu"), layout);
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
                true => Color::Green,
                false => Color::Red,
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
            Color::Green
        } else {
            Color::Red
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

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
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

            (_, KeyCode::Char('e')) => {
                self.mode = if self.mode == Mode::EDITING {
                    Mode::VIEWING
                } else {
                    Mode::EDITING
                }
            }
            (_, KeyCode::Char('v')) => self.mode = Mode::VIEWING,

            (_, KeyCode::Char('l') | KeyCode::Right) => self.next_column(),
            (_, KeyCode::Char('h') | KeyCode::Left) => self.previous_column(),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.next_row(),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.previous_row(),
            _ => {}
        }
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
            Color::Red => Color::Blue,
            Color::Blue => Color::White,
            Color::White => Color::Red,
            _ => Color::Red,
        };

        self.items[x][y].color = next_color;
    }

    fn add_new_row(&mut self) {
        let new_row = vec![
            gen_default_cell(),
            gen_default_cell(),
            gen_default_cell(),
            gen_default_cell(),
        ];

        self.items.push(new_row);
    }
    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
