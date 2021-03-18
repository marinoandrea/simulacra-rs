use crate::events::Event;

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub mod glfw;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub trait Window {
    fn new(props: WindowProps) -> Self;

    fn on_update(&mut self, event_queue: &mut Vec<Event>);

    fn get_title(&self) -> &str;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
pub type ApplicationWindow = glfw::GLFWWindow;
