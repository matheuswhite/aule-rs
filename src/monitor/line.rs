use core::str;
use gl::types::{GLchar, GLint};
use std::ffi::CString;

const VERTEX_SHADER: &str = include_str!("../../shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/fragment.glsl");

pub struct Lines {
    vertices: Vec<f32>,
    vao: u32,
    vbo: u32,
    program: u32,
}

impl Lines {
    pub fn new() -> Self {
        let vertex_shader = compile_shader(VERTEX_SHADER, gl::VERTEX_SHADER).unwrap();
        let fragment_shader = compile_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER).unwrap();
        let program = link_shaders(vertex_shader, fragment_shader).unwrap();

        Self {
            vertices: vec![],
            vao: 0,
            vbo: 0,
            program,
        }
    }

    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    pub fn add_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.vertices.extend_from_slice(&[x1, y1, x2, y2]);
    }

    pub fn draw(&mut self, color: [f32; 3]) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = 2 * std::mem::size_of::<f32>() as i32;

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            gl::UseProgram(self.program);
            gl::Uniform3fv(
                gl::GetUniformLocation(self.program, "color\0".as_ptr() as *const GLchar),
                1,
                &color[0],
            );
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, (self.vertices.len() / 2) as i32);
        }
    }
}

fn compile_shader(src: &str, shader_type: u32) -> Result<u32, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(src.as_bytes()).unwrap();
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );

            return Err(format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            ));
        }

        Ok(shader)
    }
}

fn link_shaders(vertex_shader: u32, fragment_shader: u32) -> Result<u32, String> {
    unsafe {
        let shader_program = gl::CreateProgram();
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        let res = if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );

            Err(format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            ))
        } else {
            Ok(shader_program)
        };
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        res
    }
}
