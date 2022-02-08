use std::{ffi::CString, mem::size_of};

use gl::types::*;
use glam::Mat4;

pub struct Camera {
	ubo: GLuint,
}

impl Camera {
	pub unsafe fn new() -> Self {
		let mut ubo = 0;
		gl::GenBuffers(1, &mut ubo);
		gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
		gl::BufferData(
			gl::UNIFORM_BUFFER,
			size_of::<Mat4>() as GLintptr * 2,
			std::ptr::null(),
			gl::STATIC_DRAW,
		);
		gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
		gl::BindBufferBase(
			gl::UNIFORM_BUFFER,
			0,
			ubo
		);
		Self { ubo }
	}

	pub unsafe fn resize(&self, w: f32, h: f32) {
		let projection = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);
		gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
		gl::BufferSubData(
			gl::UNIFORM_BUFFER,
			0,
			size_of::<Mat4>() as GLintptr,
			bytemuck::bytes_of(&projection.to_cols_array()).as_ptr() as *const _
		);
		gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
	}

	pub unsafe fn view(&self, transform: Mat4) {
		gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
		gl::BufferSubData(
			gl::UNIFORM_BUFFER,
			size_of::<Mat4>() as GLintptr,
			size_of::<Mat4>() as GLintptr,
			bytemuck::bytes_of(&transform.to_cols_array()).as_ptr() as *const _
		);
	}
}

impl Drop for Camera {
	fn drop(&mut self) {
	    unsafe {
			gl::DeleteBuffers(1, &mut self.ubo);
		}
	}
}





pub struct Shader {
	pub program: GLuint,
}

impl Shader {
	pub unsafe fn new<S: Into<String>>(
		vert: S,
		frag: S,
	) -> Self {
		let program = gl::CreateProgram();

		let version = "#version 430".to_string();
		let vert = CString::new(format!("{}\n{}", version, vert.into())).unwrap();
		let frag = CString::new(format!("{}\n{}", version, frag.into())).unwrap();

		let shaders = [
			(gl::VERTEX_SHADER  , vert),
			(gl::FRAGMENT_SHADER, frag),
		];

		let shaders = shaders.iter().map(|(shader_type, shader_source)| {
			let shader = gl::CreateShader(*shader_type);
			gl::ShaderSource(shader, 1, &shader_source.as_ptr(), std::ptr::null());
			gl::CompileShader(shader);
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

			if status != gl::TRUE as GLint {
				let mut length = 0;
				gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
				let mut log = String::with_capacity(length as usize);
				log.extend(std::iter::repeat('\0').take(length as usize));
				gl::GetShaderInfoLog(
					shader,
					length,
					&mut length,
					log.as_ptr() as *mut GLchar
				);
				log.truncate(length as usize);
				panic!("{}", log);
			}
			gl::AttachShader(program, shader);
			shader
		}).collect::<Vec<_>>();

		gl::LinkProgram(program);
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
		if status != gl::TRUE as GLint {
			let mut length = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
			let mut log = String::with_capacity(length as usize);
			log.extend(std::iter::repeat('\0').take(length as usize));
			gl::GetProgramInfoLog(
				program,
				length,
				&mut length,
				log.as_ptr() as *mut GLchar
			);
			log.truncate(length as usize);
			panic!("{}", log);
		}

		for shader in shaders {
			gl::DetachShader(program, shader);
			gl::DeleteShader(shader);
		}

		let camera = CString::new("camera").unwrap();
		let index = gl::GetUniformBlockIndex(
			program,
			camera.as_ptr()
		);
		gl::UniformBlockBinding(program, index, 0);

		Self { program }
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.program);
		}
	}
}

