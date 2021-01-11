mod helper;
use gl::types::*;

pub struct Shader {
	program: GLuint
}

#[allow(dead_code)]
impl Shader {
	pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
		let vertex_shader = helper::create_shader(gl::VERTEX_SHADER, vertex_shader_source);
		let fragment_shader = helper::create_shader(gl::FRAGMENT_SHADER, fragment_shader_source);
		let program = helper::create_program(vertex_shader, fragment_shader);
		helper::check_shader(vertex_shader);
		helper::check_shader(fragment_shader);
		helper::check_program(program);

		unsafe {
			gl::DeleteShader(vertex_shader);
			gl::DeleteShader(fragment_shader);
		}

		Shader {
			program,
		}
	}

	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.program);
		}
	}

	pub fn set_int(&self, name: &str, value: i32) {
		let name = std::ffi::CString::new(name).unwrap();
		unsafe {
			self.use_program();
			gl::Uniform1i(gl::GetUniformLocation(self.program, name.as_ptr()), value);
		}
	}

	pub fn set_matrix4(&self, name: &str, value: *const GLfloat) {
		let name = std::ffi::CString::new(name).unwrap();
		unsafe {
			self.use_program();
			gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, name.as_ptr()), 1, gl::FALSE, value);
		}
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.program); }
		self.program = 0;
	}
}
