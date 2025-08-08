use crate::monitor::line::Lines;
use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use core::str;
use glfw::{Action, Context, GlfwReceiver, Key};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{collections::HashMap, rc::Rc, sync::Mutex};

struct WindowContext {
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, glfw::WindowEvent)>,
}

pub struct PlotterContext {
    glfw: glfw::Glfw,
    windows: HashMap<u64, WindowContext>,
    counter: u64,
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

        Rc::new(Mutex::new(Self {
            glfw,
            windows: HashMap::new(),
            counter: 0,
        }))
    }

    pub fn new_window(&mut self, title: &str, width: u32, height: u32) -> u64 {
        let id = self.counter;
        self.counter += 1;

        let (mut window, events) = self
            .glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        self.windows.insert(id, WindowContext { window, events });

        id
    }

    pub fn run<F>(&mut self, id: u64, mut draw_fn: F)
    where
        F: FnMut(),
    {
        let Some(WindowContext { window, events }) = self.windows.get_mut(&id) else {
            return;
        };

        if window.should_close() {
            return;
        }

        window.make_current();

        Self::process_events(window, events);

        draw_fn();

        window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn run_without_draw(&mut self, id: u64) {
        let Some(WindowContext { window, events }) = self.windows.get_mut(&id) else {
            return;
        };

        if window.should_close() {
            return;
        }

        window.make_current();
        Self::process_events(window, events);

        self.glfw.poll_events();
    }

    fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

pub fn keep_alive(plotter_context: Rc<Mutex<PlotterContext>>) {
    loop {
        let mut guard = plotter_context.lock().unwrap();
        if guard.windows.is_empty() {
            break;
        }

        guard
            .windows
            .retain(|_, window_context| !window_context.window.should_close());

        for id in guard.windows.keys().cloned().collect::<Vec<_>>() {
            guard.run_without_draw(id);
        }

        sleep(Duration::from_millis(16)); // Roughly 60 FPS
    }
}

pub struct Plotter {
    context: Rc<Mutex<PlotterContext>>,
    lines: Lines,
    id: u64,
    sim_time: f32,
    max: (f32, f32),
    min: (f32, f32),
}

pub struct RTPlotter {
    context: Rc<Mutex<PlotterContext>>,
    lines: Lines,
    id: u64,
    sim_time: f32,
    max: (f32, f32),
    min: (f32, f32),
    last_update: Instant,
}

impl Plotter {
    pub fn new(
        title: &str,
        x_limits: (f32, f32),
        y_limits: (f32, f32),
        context: &Rc<Mutex<PlotterContext>>,
    ) -> Self {
        let id = {
            let mut guard = context.lock().unwrap();
            guard.new_window(title, 400, 300)
        };

        Self {
            context: context.clone(),
            id,
            lines: Lines::new(),
            sim_time: 0.0,
            max: (x_limits.1, y_limits.1),
            min: (x_limits.0, y_limits.0),
        }
    }

    fn add_point(&mut self, x: f32, y: f32) {
        let x = (x - self.min.0) / (self.max.0 - self.min.0) * 2.0 - 1.0; // Normalize to [-1, 1]
        let y = (y - self.min.1) / (self.max.1 - self.min.1) * 2.0 - 1.0; // Normalize to [-1, 1]
        let vertices = self.lines.vertices();

        let (last_x, last_y) = if vertices.len() >= 2 {
            (vertices[vertices.len() - 2], vertices[vertices.len() - 1])
        } else {
            (x, y)
        };

        self.lines.add_line(last_x, last_y, x, y);
    }

    pub fn display(&mut self) {
        let mut guard = self.context.lock().unwrap();
        guard.run(self.id, || unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.lines.draw([1.0, 0.0, 0.0]);
        });
    }
}

impl RTPlotter {
    pub fn new(
        title: &str,
        x_limits: (f32, f32),
        y_limits: (f32, f32),
        context: &Rc<Mutex<PlotterContext>>,
    ) -> Self {
        let id = {
            let mut guard = context.lock().unwrap();
            guard.new_window(title, 400, 300)
        };

        Self {
            last_update: Instant::now(),
            context: context.clone(),
            id,
            lines: Lines::new(),
            sim_time: 0.0,
            max: (x_limits.1, y_limits.1),
            min: (x_limits.0, y_limits.0),
        }
    }

    fn add_point(&mut self, x: f32, y: f32) {
        let x = (x - self.min.0) / (self.max.0 - self.min.0) * 2.0 - 1.0; // Normalize to [-1, 1]
        let y = (y - self.min.1) / (self.max.1 - self.min.1) * 2.0 - 1.0; // Normalize to [-1, 1]
        let vertices = self.lines.vertices();

        let (last_x, last_y) = if vertices.len() >= 2 {
            (vertices[vertices.len() - 2], vertices[vertices.len() - 1])
        } else {
            (x, y)
        };

        self.lines.add_line(last_x, last_y, x, y);
    }
}

impl Monitor for Plotter {
    fn show(&mut self, input: Vec<Signal>) {
        self.sim_time += input[0].dt.as_secs_f32();
        self.add_point(self.sim_time, input[0].value);
    }
}

impl Monitor for RTPlotter {
    fn show(&mut self, input: Vec<Signal>) {
        self.sim_time += input[0].dt.as_secs_f32();
        self.add_point(self.sim_time, input[0].value);

        if Instant::now().duration_since(self.last_update) < Duration::from_millis(17) {
            return;
        }
        self.last_update = Instant::now();

        let mut guard = self.context.lock().unwrap();
        guard.run(self.id, || unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.lines.draw([1.0, 0.0, 0.0]);
        });
    }
}

impl AsMonitor for Plotter {}

impl AsMonitor for RTPlotter {}
