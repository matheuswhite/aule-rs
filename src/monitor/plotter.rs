use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use core::str;
use gl::types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr};
use glfw::{Action, Context, GlfwReceiver, Key};
use std::{collections::HashMap, ffi::CString, os::raw::c_void, rc::Rc, sync::Mutex};

const VERTEX_SHADER: &str = include_str!("../../shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/fragment.glsl");

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

    pub fn run(&mut self, id: u64) {
        let Some(WindowContext { window, events }) = self.windows.get_mut(&id) else {
            return;
        };

        if window.should_close() {
            return;
        }

        window.make_current();

        process_events(window, events);

        // TODO: rendering logic
        // ---------------------------------------

        window.swap_buffers();
        self.glfw.poll_events();
    }
}

pub fn keep_alive(plotter_context: Rc<Mutex<PlotterContext>>) {
    let mut guard = plotter_context.lock().unwrap();

    while !guard.windows.is_empty() {
        guard
            .windows
            .retain(|_, window_context| !window_context.window.should_close());

        for id in guard.windows.keys().cloned().collect::<Vec<_>>() {
            guard.run(id);
        }
    }
}

pub struct Plotter {
    context: Rc<Mutex<PlotterContext>>,
    id: u64,
    program: (u32, u32),
}

impl Plotter {
    pub fn new(title: &str, context: &Rc<Mutex<PlotterContext>>) -> Self {
        let id = {
            let mut guard = context.lock().unwrap();
            guard.new_window(title, 400, 300)
        };

        Plotter {
            context: context.clone(),
            id,
            program: create_shader_program(VERTEX_SHADER, FRAGMENT_SHADER),
        }
    }
}

impl Monitor for Plotter {
    fn show(&mut self, _input: Vec<Signal>) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            gl::UseProgram(self.program.0);
            gl::BindVertexArray(self.program.1);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }

        let mut guard = self.context.lock().unwrap();
        guard.run(self.id);
    }
}

impl AsMonitor for Plotter {}

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

fn create_shader_program(vertex_shader_source: &str, fragment_shader_source: &str) -> (u32, u32) {
    unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);
        // check for shader compile errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];
        let (mut VBO, mut VAO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, VAO)
    }
}
