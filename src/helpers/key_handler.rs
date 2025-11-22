use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
use ratatui::style::Color;

use crate::{
    App, COLOR_BLUE, COLOR_BLUE_ACTIVE, COLOR_RED, COLOR_RED_ACTIVE, COLOR_YELLOW,
    COLOR_YELLOW_ACTIVE, Datos, HEIGHT, Mode, Pattern, PyramidType, SymbolSize, WIDTH,
};

fn save_pattern(app: &mut App) {
    app.patterns.push(app.data_big.clone());
    // app.patterns.insert(0, app.data_big.data.clone());
    app.data_big.data = Pattern(vec![]);
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
            let pattern = if app.mode == Mode::EDITING {
                app.active_pattern()
            } else {
                app.active_viewed_pattern()
            };
            let object = pattern
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let mut new_type = PyramidType::Straight;
            let mut old_color = COLOR_BLUE;
            let mut old_size = SymbolSize::MEDIUM;
            if object.is_some() {
                old_color = object.unwrap().color;
                old_size = object.unwrap().size;
                let index_of_selected = pattern
                    .iter()
                    .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

                new_type = match object.unwrap().pyramid_type {
                    PyramidType::Straight => PyramidType::Angled,
                    _ => PyramidType::Straight,
                };

                if app.mode == Mode::EDITING {
                    app.data_big.data.remove(index_of_selected.unwrap());
                } else {
                    app.patterns
                        .get_mut(app.selected_pattern_index)
                        .unwrap()
                        .data
                        .remove(index_of_selected.unwrap());
                }
            }

            if app.mode == Mode::EDITING {
                app.data_big.data.push(Datos {
                    pyramid_type: new_type,
                    pos: app.current_pos,
                    color: old_color,
                    size: old_size,
                });
            } else {
                app.patterns
                    .get_mut(app.selected_pattern_index)
                    .unwrap()
                    .data
                    .push(Datos {
                        pyramid_type: new_type,
                        pos: app.current_pos,
                        color: old_color,
                        size: old_size,
                    });
            }
        }
        (_, KeyCode::Char('c')) => {
            let pattern = if app.mode == Mode::EDITING {
                app.active_pattern()
            } else {
                app.active_viewed_pattern()
            };
            let object = pattern
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let index_of_selected = pattern
                .iter()
                .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            let new_color = match object.unwrap().color {
                COLOR_RED => COLOR_BLUE,
                COLOR_BLUE => COLOR_YELLOW,
                COLOR_YELLOW => COLOR_RED,
                _ => COLOR_RED,
            };
            let old_type = object.unwrap().pyramid_type;
            let old_size = object.unwrap().size;

            if app.mode == Mode::EDITING {
                app.data_big.data.remove(index_of_selected.unwrap());

                app.data_big.data.push(Datos {
                    pyramid_type: old_type,
                    pos: app.current_pos,
                    color: new_color,
                    size: old_size,
                });
            } else {
                app.patterns
                    .get_mut(app.selected_pattern_index)
                    .unwrap()
                    .data
                    .remove(index_of_selected.unwrap());

                app.patterns
                    .get_mut(app.selected_pattern_index)
                    .unwrap()
                    .data
                    .push(Datos {
                        pyramid_type: old_type,
                        pos: app.current_pos,
                        color: new_color,
                        size: old_size,
                    });
            }
        }
        (_, KeyCode::Char('s')) => {
            let pattern = if app.mode == Mode::EDITING {
                app.active_pattern()
            } else {
                app.active_viewed_pattern()
            };

            let object = pattern
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            if object.is_none() {
                return;
            }
            let index_of_selected = pattern
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

            let pattern = if app.mode == Mode::EDITING {
                app.data_big.data.remove(index_of_selected.unwrap());

                app.data_big.data.push(Datos {
                    pyramid_type: old_type,
                    pos: app.current_pos,
                    color: old_color,
                    size: new_size,
                });
            } else {
                app.patterns
                    .get_mut(app.selected_pattern_index)
                    .unwrap()
                    .data
                    .remove(index_of_selected.unwrap());

                app.patterns
                    .get_mut(app.selected_pattern_index)
                    .unwrap()
                    .data
                    .push(Datos {
                        pyramid_type: old_type,
                        pos: app.current_pos,
                        color: old_color,
                        size: new_size,
                    });
            };
        }
        (_, KeyCode::Backspace) => {
            let object = app
                .active_pattern()
                .iter()
                .find(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            if object.is_none() {
                return;
            }
            let index_of_selected = app
                .active_pattern()
                .iter()
                .position(|d| d.pos.0 == app.current_pos.0 && d.pos.1 == app.current_pos.1);

            app.data_big.data.remove(index_of_selected.unwrap());
        }

        (_, KeyCode::Enter) => {
            save_pattern(app);
            app.pattern_index = app.patterns.len() - 1;
            app.selected_pattern_index = app.patterns.len() - 1;
            app.render_end_index = app.pattern_index;
            step_right(app, 1);
        }

        (_, KeyCode::Char('a')) => {
            step_left(app, 1);
        }

        (_, KeyCode::Char('d')) => {
            step_right(app, 1);
        }

        (_, KeyCode::Char('m')) => {
            modify_pattern(app);
        }

        // modes
        (_, KeyCode::Char('e')) => app.mode = crate::Mode::EDITING,
        (_, KeyCode::Char('v')) => app.mode = crate::Mode::VIEWING,
        (_, KeyCode::Char('>')) => {
            app.pattern_rows = (app.pattern_rows - 1).max(1);
            step_right(app, 1);
            step_left(app, 1);
        }
        (_, KeyCode::Char('<')) => {
            app.patterns_per_row = (app.patterns_per_row - 1).max(1);
            step_right(app, 1);
            step_left(app, 1);
        }
        (_, KeyCode::Char('.')) => {
            app.pattern_rows = app.pattern_rows + 1;
            step_right(app, 1);
            step_left(app, 1);
        }
        (_, KeyCode::Char(',')) => {
            app.patterns_per_row = app.patterns_per_row + 1;
            step_right(app, 1);
            step_left(app, 1);
        }

        (_, KeyCode::Char('i')) => {
            if app.mode == Mode::EDITING {
                app.data_big.valid = !app.data_big.valid;
            } else {
                let asd = app.patterns.get_mut(app.selected_pattern_index).unwrap();
                asd.valid = !asd.valid;
            }
        }
        (_, KeyCode::Char('A')) => {
            let old_index = app.selected_pattern_index;
            let old_index_modulo = app.selected_pattern_index % app.patterns_per_row;
            app.selected_pattern_index = app.selected_pattern_index.saturating_sub(1);
            if (app.selected_pattern_index % app.patterns_per_row > old_index_modulo) {
                app.selected_pattern_index = old_index;
            }
        }
        (_, KeyCode::Char('D')) => {
            let old_index = app.selected_pattern_index;
            let old_index_modulo = app.selected_pattern_index % app.patterns_per_row;
            app.selected_pattern_index = (app.selected_pattern_index + 1);
            if (app.selected_pattern_index % app.patterns_per_row == 0) {
                app.selected_pattern_index = old_index;
            }
        }
        (_, KeyCode::Char('W')) => {
            let old_index = app.selected_pattern_index;
            let old_index_modulo = app.selected_pattern_index % app.patterns_per_row;
            app.selected_pattern_index = app
                .selected_pattern_index
                .saturating_sub(app.patterns_per_row);
            if (app.selected_pattern_index % app.patterns_per_row != old_index_modulo) {
                app.selected_pattern_index = old_index;
            }

            if app.selected_pattern_index < app.render_start_index {
                step_left(app, app.patterns_per_row);
            }
        }
        (_, KeyCode::Char('S')) => {
            let old_index = app.selected_pattern_index;
            let old_index_modulo = app.selected_pattern_index % app.patterns_per_row;

            app.selected_pattern_index = (app.selected_pattern_index + app.patterns_per_row);

            if ((app.selected_pattern_index % app.patterns_per_row != old_index_modulo)
                || (app.selected_pattern_index > app.patterns.len().saturating_sub(1)))
            {
                app.selected_pattern_index = old_index;
            }

            if app.selected_pattern_index > app.render_end_index {
                step_right(app, app.patterns_per_row);
            }
        }
        _ => {}
    }
}

fn step_left(app: &mut App, step_size: usize) {
    let select_size = app.pattern_rows * app.patterns_per_row;
    let min_index = select_size.min(app.patterns.len()).saturating_sub(1);

    app.render_end_index = (app.render_end_index.saturating_sub(step_size)).max(min_index);
    app.render_start_index = app.render_end_index.saturating_sub(select_size - 1);
}

fn step_right(app: &mut App, step_size: usize) {
    let select_size = app.pattern_rows * app.patterns_per_row;

    app.render_end_index = (app.render_end_index + step_size).min(app.patterns.len() - 1);
    app.render_start_index = app.render_end_index.saturating_sub(select_size - 1);
}

fn modify_pattern(app: &mut App) {
    app.patterns[app.pattern_index] = app.data_big.clone();
}

fn set_pattern(app: &mut App) {
    let last_pattern = app.patterns.get(app.pattern_index);

    let next_pattern = match last_pattern {
        Some(last_pattern) => last_pattern.clone(),
        _ => crate::PatternParent {
            data: Pattern(vec![]),
            valid: true,
        },
    };

    app.data_big = next_pattern;
}
