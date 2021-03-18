use glfw::Context;

use crate::{log_info, window::glfw::GLFWWindow};

pub trait RenderingContext {
    fn init(&mut self);
    fn swap_buffers(&mut self);
}

impl RenderingContext for GLFWWindow {
    fn init(&mut self) {
        self.window.make_current();

        gl::load_with(|s| self.window.get_proc_address(s) as *const _);

        log_info!(
            "OpenGL Version: {:?}.{:?}",
            gl::MAJOR_VERSION,
            gl::MINOR_VERSION
        );
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
