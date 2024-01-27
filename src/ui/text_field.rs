use bevy::{
    input::{keyboard::KeyCode, keyboard::KeyboardInput},
    prelude::*,
};

use crate::GameState;

const TRANSPARENT_BACKGROUND: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);
const FONT_SIZE_INPUT: f32 = 32.0;
const CHAR_SIZE: f32 = 2.5;
const CHAR_OFFSET: f32 = 1.5;
const CHAR_PIXEL_FACTOR: f32 = 12.8;
const MAX_CHAR_COUNT: usize = 16;

#[derive(Resource, Default, Debug)]
pub struct TypingState {
    buf: String,
    just_typed_char: bool,
}

#[derive(Component)]
pub struct InputField;
#[derive(Component)]
struct TypingBuffer;
#[derive(Component)]
struct TypingCursor;
#[derive(Resource)]
struct TypingCursorTimer(Timer);

#[derive(Event)]
pub struct SubmittedTextInput(pub String);

fn trim_last_word(s: &str) -> String {
    let trimmed_str = s.trim_end();
    match trimmed_str.rfind(' ') {
        Some(space_index) => trimmed_str[..space_index + 1].to_string(),
        None => String::new(),
    }
}

pub fn spawn_text_field(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let input_pointer = commands
        .spawn(TextBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                ">".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE_INPUT,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .id();

    let text = commands
        .spawn((
            TypingBuffer,
            TextBundle {
                text: Text::from_section(
                    "".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: FONT_SIZE_INPUT,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },
        ))
        .id();

    let cursor = commands
        .spawn((
            TypingCursor,
            TextBundle {
                text: Text::from_section(
                    "_".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: FONT_SIZE_INPUT,
                        color: Color::RED,
                    },
                ),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            InputField,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    width: Val::Px((2.0 * CHAR_SIZE + CHAR_OFFSET) * CHAR_PIXEL_FACTOR),
                    height: Val::Px(42.0),
                    ..default()
                },
                background_color: TRANSPARENT_BACKGROUND.into(),
                ..default()
            },
        ))
        .push_children(&[input_pointer, text, cursor])
        .id()
}

fn clear_buffer_text(mut typing_state: ResMut<TypingState>) {
    typing_state.buf.clear();
}

fn update_buffer_container(
    typing_state: Res<TypingState>,
    mut q_buffer_container: Query<&mut Style, With<InputField>>,
) {
    if !typing_state.is_changed() {
        return;
    }

    let mut style = match q_buffer_container.get_single_mut() {
        Ok(s) => s,
        Err(_) => return,
    };

    let k = 2.0 + typing_state.buf.len() as f32;
    style.width = Val::Px((k * CHAR_SIZE + CHAR_OFFSET) * CHAR_PIXEL_FACTOR);
}

fn update_buffer_text(
    typing_state: Res<TypingState>,
    mut q_typing_buffer_text: Query<&mut Text, With<TypingBuffer>>,
) {
    if !typing_state.is_changed() {
        return;
    }

    let mut text = match q_typing_buffer_text.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    text.sections[0].value.clone_from(&typing_state.buf);
}

fn update_cursor_text(
    mut timer: ResMut<TypingCursorTimer>,
    mut q_cursor: Query<&mut Text, With<TypingCursor>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for mut target in q_cursor.iter_mut() {
        if target.sections[0].style.color != Color::NONE {
            target.sections[0].style.color = Color::NONE;
        } else {
            target.sections[0].style.color = Color::RED;
        }
    }
}

fn push_chars(
    keys: Res<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut typing_state: ResMut<TypingState>,
    q_input_field: Query<With<InputField>>,
    mut submitted_text_input: EventWriter<SubmittedTextInput>,
) {
    let control_active = keys.pressed(KeyCode::ControlLeft);
    let shift_active = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    for ev in keyboard_input_events.read() {
        // We run this in the loop so that the events get consumed.
        // Otherwise we might run into the issue of adding it to the buffer
        // when spawning the text field.
        if q_input_field.is_empty() {
            continue;
        }
        if ev.state.is_pressed() {
            if ev.key_code == Some(KeyCode::Back) {
                if !control_active {
                    typing_state.buf.pop();
                } else {
                    typing_state.buf = trim_last_word(&typing_state.buf);
                }
            }
            if ev.key_code == Some(KeyCode::Return) {
                let text = typing_state.buf.clone();
                submitted_text_input.send(SubmittedTextInput(text));
                continue;
            }

            if typing_state.buf.len() >= MAX_CHAR_COUNT {
                continue;
            }

            let maybe_char = match ev.key_code {
                Some(KeyCode::A) => Some('a'),
                Some(KeyCode::B) => Some('b'),
                Some(KeyCode::C) => Some('c'),
                Some(KeyCode::D) => Some('d'),
                Some(KeyCode::E) => Some('e'),
                Some(KeyCode::F) => Some('f'),
                Some(KeyCode::G) => Some('g'),
                Some(KeyCode::H) => Some('h'),
                Some(KeyCode::I) => Some('i'),
                Some(KeyCode::J) => Some('j'),
                Some(KeyCode::K) => Some('k'),
                Some(KeyCode::L) => Some('l'),
                Some(KeyCode::M) => Some('m'),
                Some(KeyCode::N) => Some('n'),
                Some(KeyCode::O) => Some('o'),
                Some(KeyCode::P) => Some('p'),
                Some(KeyCode::Q) => Some('q'),
                Some(KeyCode::R) => Some('r'),
                Some(KeyCode::S) => Some('s'),
                Some(KeyCode::T) => Some('t'),
                Some(KeyCode::U) => Some('u'),
                Some(KeyCode::V) => Some('v'),
                Some(KeyCode::W) => {
                    if !control_active {
                        Some('w')
                    } else {
                        typing_state.buf = trim_last_word(&typing_state.buf);
                        None
                    }
                }
                Some(KeyCode::X) => Some('x'),
                Some(KeyCode::Y) => Some('y'),
                Some(KeyCode::Z) => Some('z'),
                Some(KeyCode::Space) => Some('_'),
                Some(KeyCode::Minus) => Some('-'),
                _ => None,
            };

            if let Some(char) = maybe_char {
                let char = if shift_active {
                    char.to_uppercase()
                        .to_string()
                        .chars()
                        .next()
                        .unwrap_or(char)
                } else {
                    char
                };
                typing_state.buf.push(char);
                typing_state.just_typed_char = true;
            } else {
                typing_state.just_typed_char = false;
            }
        }
    }
}

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TypingCursorTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .init_resource::<TypingState>()
        .add_event::<SubmittedTextInput>()
        .add_systems(OnEnter(GameState::GameOver), clear_buffer_text)
        .add_systems(
            Update,
            (
                push_chars,
                update_cursor_text,
                update_buffer_container.after(push_chars),
                update_buffer_text.after(push_chars),
            ),
        );
    }
}
