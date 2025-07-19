use std::{rc::Rc, sync::Mutex};

use crate::block::{AsMonitor, Monitor, Signal};
use glfw::{Action, Context, GlfwReceiver, Key};

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment.glsl");

pub struct PlotterContext {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl PlotterContext {
    pub fn new() -> Rc<Mutex<Self>> {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        // --------------------
        let (mut window, events) = glfw
            .create_window(400, 300, "LearnOpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Rc::new(Mutex::new(Self {
            glfw,
            window,
            events,
        }))
    }

    pub fn run(&mut self) {
        if self.window.should_close() {
            return;
        }

        // events
        // -----
        process_events(&mut self.window, &self.events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        self.window.swap_buffers();
        self.glfw.poll_events();
    }
}

pub fn keep_alive(plotter_context: Rc<Mutex<PlotterContext>>) {
    let mut guard = plotter_context.lock().unwrap();

    while !guard.window.should_close() {
        guard.run();
    }
}

pub struct Plotter {
    title: String,
    unit: String,
    context: Rc<Mutex<PlotterContext>>,
}

impl Plotter {
    pub fn new(title: &str, unit: &str, context: &Rc<Mutex<PlotterContext>>) -> Self {
        Plotter {
            title: title.to_string(),
            unit: unit.to_string(),
            context: context.clone(),
        }
    }
}

impl Monitor for Plotter {
    fn show(&mut self, _input: Signal) {
        // todo!()

        let mut guard = self.context.lock().unwrap();
        guard.run();
    }
}

impl AsMonitor for Plotter {}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
