use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use ratatui::style::Color;

use crate::{App, Datos, HEIGHT, Pattern, PyramidType, SymbolSize, WIDTH};

fn save_pattern(app: &mut App) {
    app.patterns.push(app.data_big.clone());
}

pub fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q')) => app.quit(),

        // Select Directions
        (_, KeyCode::Right) => {
            if app.current_pos.0 > WIDTH - 2 {
                app.current_pos.0 = 0
            } else {
                app.current_pos.0 = app.current_pos.0 + 1
            }
        }
        (_, KeyCode::Left) => {
            if app.current_pos.0 == 0 {
                app.current_pos.0 = WIDTH - 1
            } else {
                app.current_pos.0 = app.current_pos.0 - 1
            }
        }
        (_, KeyCode::Down) => {
            if app.current_pos.1 > HEIGHT - 2 {
                app.current_pos.1 = 0
            } else {
                app.current_pos.1 = app.current_pos.1 + 1
            }
        }
        (_, KeyCode::Up) => {
            if app.current_pos.1 == 0 {
                app.current_pos.1 = HEIGHT - 1
            } else {
                app.current_pos.1 = app.current_pos.1 - 1
            }
        }

        // Symbol Manipulation
        (_, KeyCode::Char('r')) => {
            let object = app
                .data_big
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let mut new_type = PyramidType::Straight;
            let mut old_color = Color::LightGreen;
            let mut old_size = SymbolSize::MEDIUM;
            if object.is_some() {
                old_color = object.unwrap().color;
                old_size = object.unwrap().size;
                let index_of_selected = app
                    .data_big
                    .iter()
                    .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

                new_type = match object.unwrap().pyramid_type {
                    PyramidType::Straight => PyramidType::Angled,
                    _ => PyramidType::Straight,
                };

                app.data_big.remove(index_of_selected.unwrap());
            }

            app.data_big.push(Datos {
                pyramid_type: new_type,
                pos: app.current_pos,
                color: old_color,
                size: old_size,
            });
        }
        (_, KeyCode::Char('c')) => {
            let object = app
                .data_big
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            if object.is_none() {
                return;
            }
            let index_of_selected = app
                .data_big
                .iter()
                .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let new_color = match object.unwrap().color {
                Color::LightRed => Color::LightGreen,
                Color::LightGreen => Color::LightYellow,
                Color::LightYellow => Color::LightRed,
                _ => Color::LightRed,
            };
            let old_type = object.unwrap().pyramid_type;
            let old_size = object.unwrap().size;
            app.data_big.remove(index_of_selected.unwrap());

            app.data_big.push(Datos {
                pyramid_type: old_type,
                pos: app.current_pos,
                color: new_color,
                size: old_size,
            });
        }
        (_, KeyCode::Char('s')) => {
            let object = app
                .data_big
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            if object.is_none() {
                return;
            }
            let index_of_selected = app
                .data_big
                .iter()
                .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let new_size = match object.unwrap().size {
                SymbolSize::SMALL => SymbolSize::MEDIUM,
                SymbolSize::MEDIUM => SymbolSize::LARGE,
                SymbolSize::LARGE => SymbolSize::SMALL,
                _ => SymbolSize::MEDIUM,
            };
            let old_type = object.unwrap().pyramid_type;
            let old_color = object.unwrap().color;
            app.data_big.remove(index_of_selected.unwrap());

            app.data_big.push(Datos {
                pyramid_type: old_type,
                pos: app.current_pos,
                color: old_color,
                size: new_size,
            });
        }
        (_, KeyCode::Backspace) => {
            let object = app
                .data_big
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            if object.is_none() {
                return;
            }
            let index_of_selected = app
                .data_big
                .iter()
                .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            app.data_big.remove(index_of_selected.unwrap());
        }

        (_, KeyCode::Enter) => {
            save_pattern(app);
            app.pattern_index = app.patterns.len() - 1;
        }

        (/*KeyModifiers::SHIFT*/ _, KeyCode::Char('p')) => {
            app.pattern_index = if app.pattern_index > 0 {
                app.pattern_index - 1
            } else {
                0
            };
            set_pattern(app);
        }

        (_, KeyCode::Char('i')) => {
            app.pattern_index = (app.pattern_index + 1).min(app.patterns.len() - 1);
            set_pattern(app);
        }

        (_, KeyCode::Char('m')) => {
            set_pattern(app);
        }

        //
        _ => {}
    }
}

fn set_pattern(app: &mut App) {
    let last_pattern = app.patterns.get(app.pattern_index);

    let next_pattern = match last_pattern {
        Some(last_pattern) => last_pattern.clone(),
        _ => Pattern(vec![Datos {
            pos: (2, 2),
            pyramid_type: PyramidType::Straight,
            color: Color::Red,
            size: SymbolSize::LARGE,
        }]),
    };

    app.data_big = next_pattern;
}
