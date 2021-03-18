use crate::layers::Layer;
use crate::logger;
use crate::renderer::shader::{OpenGLShader, Shader};
use crate::window::{ApplicationWindow, Window, WindowProps};
use crate::{events::Event, renderer::context::RenderingContext};
use crate::{log_info, log_trace};

static VERTICES: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

static VERTEX_SHADER: &'static str = "
    #version 330 core

    layout(location = 0) in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
";

static FRAGMENT_SHADER: &'static str = "
    #version 330 core

    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(0.8, 0.2, 0.3, 1.0);
    }
";

pub struct Application {
    is_initialized: bool,
    is_running: bool,
    event_queue: Vec<Event>,
    layer_stack: Vec<Box<dyn Layer>>,
    window: ApplicationWindow,
}

impl Application {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            is_running: false,
            event_queue: Vec::new(),
            layer_stack: Vec::new(),
            window: ApplicationWindow::new(WindowProps {
                title: "Simulacra".to_string(),
                width: 1024,
                height: 728,
            }),
        }
    }

    pub fn init(&mut self) {
        logger::init();
        log_info!("App Starting");

        self.window.init();

        self.is_initialized = true;
        self.is_running = true;
    }

    pub fn run(&mut self) {
        if !self.is_initialized {
            panic!("You must initialize the application first.")
        }

        let (vao, _) = gl_test_setup();

        while self.is_running {
            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            for layer in self.layer_stack.iter_mut().rev() {
                (*layer).handle_events(&mut self.event_queue);
            }

            self.window.on_update(&mut self.event_queue);
            self.handle_events();
        }
    }

    fn handle_events(&mut self) {
        for e in self.event_queue.iter() {
            match e {
                Event::WindowClose => {
                    log_info!("App Stopping");
                    self.is_running = false;
                }
                _ => (),
            };
        }
        // reset event queue
        self.event_queue.clear();
    }
}

pub fn gl_test_setup() -> (u32, u32) {
    use gl::types::*;
    use std::{ffi::CString, mem, ptr};

    let mut vao = 0;
    let mut vbo = 0;

    let shader = OpenGLShader::new(VERTEX_SHADER, FRAGMENT_SHADER);

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&VERTICES[0]),
            gl::STATIC_DRAW,
        );

        // Use shader program
        shader.bind();
        gl::BindFragDataLocation(
            shader.get_program_id(),
            0,
            CString::new("color").unwrap().as_ptr(),
        );

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(
            shader.get_program_id(),
            CString::new("position").unwrap().as_ptr(),
        );
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );
    }

    (vao, vbo)
}
