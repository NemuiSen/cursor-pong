use std::ffi::CString;
use gl::types::*;

pub fn create_shader(shader_type: GLenum, source: &str) -> GLuint {
	unsafe {
		let shader = gl::CreateShader(shader_type);
		let c_source = CString::new(source.as_bytes()).unwrap();
		gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());
		gl::CompileShader(shader);

		shader
	}
}

pub fn check_shader(shader: GLuint) {
	unsafe {
		let mut success = gl::FALSE as GLint;
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(511);
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(
				shader,
				512,
				std::ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar
			);

			let mut shader_type: GLint = 0;
			gl::GetShaderiv(shader, gl::SHADER_TYPE, &mut shader_type);

			let str_shader_type: &str;
			if shader_type as GLenum == gl::VERTEX_SHADER { str_shader_type = "VERTEX" }
			else { str_shader_type = "FRAGMENT" }

			println!("[ERROR::SHADER::{}]: Compilation failed\n{}", str_shader_type, std::str::from_utf8(&info_log).unwrap());
		}
	}
}

pub fn create_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
	unsafe {
		let program = gl::CreateProgram();
		gl::AttachShader(program, vertex_shader);
		gl::AttachShader(program, fragment_shader);
		gl::LinkProgram(program);

		program
	}
}

pub fn check_program(program: GLuint) {
	unsafe {
		let mut success = gl::FALSE as GLint;
		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(511);
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetProgramInfoLog(program, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			println!("[ERROR::SHADER::PROGRAM]: Compilation Failed\n{}", std::str::from_utf8(&info_log).unwrap());
		}
	}
}
