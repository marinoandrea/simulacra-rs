use std::sync::mpsc::Receiver;

use crate::{
    events::{Event, Key, MouseButton},
    renderer::context::RenderingContext,
    window::{Window, WindowProps},
};
use glfw;

pub struct GLFWWindow {
    props: WindowProps,
    glfw: glfw::Glfw,
    pub window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window for GLFWWindow {
    fn new(props: WindowProps) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

        let (mut window, event_receiver) = glfw
            .create_window(
                props.width,
                props.height,
                &props.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);

        GLFWWindow {
            glfw,
            window,
            event_receiver,
            props,
        }
    }

    fn get_title(&self) -> &str {
        &self.props.title
    }

    fn get_width(&self) -> u32 {
        self.props.width
    }

    fn get_height(&self) -> u32 {
        self.props.height
    }

    fn on_update(&mut self, event_queue: &mut Vec<Event>) {
        if self.window.should_close() {
            event_queue.push(Event::WindowClose);
            return;
        }

        self.swap_buffers();
        self.glfw.poll_events();

        for (_, glfw_event) in glfw::flush_messages(&self.event_receiver) {
            let internal_event: Option<Event> = match glfw_event {
                glfw::WindowEvent::Pos(x, y) => Some(Event::WindowMoved {
                    x: x as u32,
                    y: y as u32,
                }),

                glfw::WindowEvent::Size(width, height) => Some(Event::WindowResize {
                    width: width as u32,
                    height: height as u32,
                }),

                glfw::WindowEvent::Close => Some(Event::WindowClose),

                glfw::WindowEvent::Focus(true) => Some(Event::WindowFocus),
                glfw::WindowEvent::Focus(false) => Some(Event::WindowLostFocus),

                glfw::WindowEvent::CursorPos(x, y) => Some(Event::MouseMoved {
                    x: x as u64,
                    y: y as u64,
                }),

                glfw::WindowEvent::Scroll(x, y) => Some(Event::MouseScrolled {
                    x: x as u64,
                    y: y as u64,
                }),

                glfw::WindowEvent::Key(key, _, action, _) => {
                    let internal_key = map_glfw_key_to_internal(key);
                    if let Some(ik) = internal_key {
                        match action {
                            glfw::Action::Press => Some(Event::KeyPressed { key: ik }),
                            glfw::Action::Release => Some(Event::KeyReleased { key: ik }),
                            _ => None,
                        };
                    };
                    None
                }

                glfw::WindowEvent::MouseButton(btn, action, _) => {
                    let internal_btn = map_glfw_btn_to_internal(btn);
                    if let Some(ib) = internal_btn {
                        match action {
                            glfw::Action::Press => Some(Event::MouseButtonPressed { btn: ib }),
                            glfw::Action::Release => Some(Event::MouseButtonReleased { btn: ib }),
                            _ => None,
                        };
                    };
                    None
                }

                _ => None,
            };

            if let Some(e) = internal_event {
                event_queue.push(e);
            }
        }
    }
}

fn map_glfw_key_to_internal(key: glfw::Key) -> Option<Key> {
    match key {
        glfw::Key::Space => Some(Key::Space),
        glfw::Key::Apostrophe => Some(Key::Apostrophe),
        glfw::Key::Comma => Some(Key::Comma),
        glfw::Key::Minus => Some(Key::Minus),
        glfw::Key::Period => Some(Key::Period),
        glfw::Key::Slash => Some(Key::Slash),
        glfw::Key::Num0 => Some(Key::Num0),
        glfw::Key::Num1 => Some(Key::Num1),
        glfw::Key::Num2 => Some(Key::Num2),
        glfw::Key::Num3 => Some(Key::Num3),
        glfw::Key::Num4 => Some(Key::Num4),
        glfw::Key::Num5 => Some(Key::Num5),
        glfw::Key::Num6 => Some(Key::Num6),
        glfw::Key::Num7 => Some(Key::Num7),
        glfw::Key::Num8 => Some(Key::Num8),
        glfw::Key::Num9 => Some(Key::Num9),
        glfw::Key::Semicolon => Some(Key::Semicolon),
        glfw::Key::Equal => Some(Key::Equal),
        glfw::Key::A => Some(Key::A),
        glfw::Key::B => Some(Key::B),
        glfw::Key::C => Some(Key::C),
        glfw::Key::D => Some(Key::D),
        glfw::Key::E => Some(Key::E),
        glfw::Key::F => Some(Key::F),
        glfw::Key::G => Some(Key::G),
        glfw::Key::H => Some(Key::H),
        glfw::Key::I => Some(Key::I),
        glfw::Key::J => Some(Key::J),
        glfw::Key::K => Some(Key::K),
        glfw::Key::L => Some(Key::L),
        glfw::Key::M => Some(Key::M),
        glfw::Key::N => Some(Key::N),
        glfw::Key::O => Some(Key::O),
        glfw::Key::P => Some(Key::P),
        glfw::Key::Q => Some(Key::Q),
        glfw::Key::R => Some(Key::R),
        glfw::Key::S => Some(Key::S),
        glfw::Key::T => Some(Key::T),
        glfw::Key::U => Some(Key::U),
        glfw::Key::V => Some(Key::V),
        glfw::Key::W => Some(Key::W),
        glfw::Key::X => Some(Key::X),
        glfw::Key::Y => Some(Key::Y),
        glfw::Key::Z => Some(Key::Z),
        glfw::Key::LeftBracket => Some(Key::LeftBracket),
        glfw::Key::Backslash => Some(Key::Backslash),
        glfw::Key::RightBracket => Some(Key::RightBracket),
        glfw::Key::GraveAccent => Some(Key::GraveAccent),
        glfw::Key::World1 => Some(Key::World1),
        glfw::Key::World2 => Some(Key::World2),
        glfw::Key::Escape => Some(Key::Escape),
        glfw::Key::Enter => Some(Key::Enter),
        glfw::Key::Tab => Some(Key::Tab),
        glfw::Key::Backspace => Some(Key::Backspace),
        glfw::Key::Insert => Some(Key::Insert),
        glfw::Key::Delete => Some(Key::Delete),
        glfw::Key::Right => Some(Key::Right),
        glfw::Key::Left => Some(Key::Left),
        glfw::Key::Down => Some(Key::Down),
        glfw::Key::Up => Some(Key::Up),
        glfw::Key::PageUp => Some(Key::PageUp),
        glfw::Key::PageDown => Some(Key::PageDown),
        glfw::Key::Home => Some(Key::Home),
        glfw::Key::End => Some(Key::End),
        glfw::Key::CapsLock => Some(Key::CapsLock),
        glfw::Key::ScrollLock => Some(Key::ScrollLock),
        glfw::Key::NumLock => Some(Key::NumLock),
        glfw::Key::PrintScreen => Some(Key::PrintScreen),
        glfw::Key::Pause => Some(Key::Pause),
        glfw::Key::F1 => Some(Key::F1),
        glfw::Key::F2 => Some(Key::F2),
        glfw::Key::F3 => Some(Key::F3),
        glfw::Key::F4 => Some(Key::F4),
        glfw::Key::F5 => Some(Key::F5),
        glfw::Key::F6 => Some(Key::F6),
        glfw::Key::F7 => Some(Key::F7),
        glfw::Key::F8 => Some(Key::F8),
        glfw::Key::F9 => Some(Key::F9),
        glfw::Key::F10 => Some(Key::F10),
        glfw::Key::F11 => Some(Key::F11),
        glfw::Key::F12 => Some(Key::F12),
        glfw::Key::F13 => Some(Key::F13),
        glfw::Key::F14 => Some(Key::F14),
        glfw::Key::F15 => Some(Key::F15),
        glfw::Key::F16 => Some(Key::F16),
        glfw::Key::F17 => Some(Key::F17),
        glfw::Key::F18 => Some(Key::F18),
        glfw::Key::F19 => Some(Key::F19),
        glfw::Key::F20 => Some(Key::F20),
        glfw::Key::F21 => Some(Key::F21),
        glfw::Key::F22 => Some(Key::F22),
        glfw::Key::F23 => Some(Key::F23),
        glfw::Key::F24 => Some(Key::F24),
        glfw::Key::F25 => Some(Key::F25),
        glfw::Key::Kp0 => Some(Key::Kp0),
        glfw::Key::Kp1 => Some(Key::Kp1),
        glfw::Key::Kp2 => Some(Key::Kp2),
        glfw::Key::Kp3 => Some(Key::Kp3),
        glfw::Key::Kp4 => Some(Key::Kp4),
        glfw::Key::Kp5 => Some(Key::Kp5),
        glfw::Key::Kp6 => Some(Key::Kp6),
        glfw::Key::Kp7 => Some(Key::Kp7),
        glfw::Key::Kp8 => Some(Key::Kp8),
        glfw::Key::Kp9 => Some(Key::Kp9),
        glfw::Key::KpDecimal => Some(Key::KpDecimal),
        glfw::Key::KpDivide => Some(Key::KpDivide),
        glfw::Key::KpMultiply => Some(Key::KpMultiply),
        glfw::Key::KpSubtract => Some(Key::KpSubtract),
        glfw::Key::KpAdd => Some(Key::KpAdd),
        glfw::Key::KpEnter => Some(Key::KpEnter),
        glfw::Key::KpEqual => Some(Key::KpEqual),
        glfw::Key::LeftShift => Some(Key::LeftShift),
        glfw::Key::LeftControl => Some(Key::LeftControl),
        glfw::Key::LeftAlt => Some(Key::LeftAlt),
        glfw::Key::LeftSuper => Some(Key::LeftSuper),
        glfw::Key::RightShift => Some(Key::RightShift),
        glfw::Key::RightControl => Some(Key::RightControl),
        glfw::Key::RightAlt => Some(Key::RightAlt),
        glfw::Key::RightSuper => Some(Key::RightSuper),
        glfw::Key::Menu => Some(Key::Menu),
        _ => None,
    }
}

fn map_glfw_btn_to_internal(btn: glfw::MouseButton) -> Option<MouseButton> {
    match btn {
        glfw::MouseButton::Button1 => Some(MouseButton::Left),
        glfw::MouseButton::Button2 => Some(MouseButton::Right),
        glfw::MouseButton::Button3 => Some(MouseButton::Middle),
        glfw::MouseButton::Button4 => Some(MouseButton::X1),
        glfw::MouseButton::Button5 => Some(MouseButton::X2),
        _ => None,
    }
}
