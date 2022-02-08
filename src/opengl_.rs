use std::{fs, mem};
use glam::*;
use glow::*;

pub struct Camera {
	ubo: Buffer,
}

impl Camera {
	pub unsafe fn new(gl: &Context) -> Self {
		let ubo = gl.create_buffer().unwrap();
		gl.bind_buffer(UNIFORM_BUFFER, Some(ubo));
		gl.buffer_data_size(UNIFORM_BUFFER, mem::size_of::<Mat4>() as i32 * 2, DYNAMIC_DRAW);
		gl.bind_buffer_base(UNIFORM_BUFFER, 0, Some(ubo));
		gl.bind_buffer(UNIFORM_BUFFER, None);
		Self { ubo }
	}

	pub unsafe fn view(&self, gl: &Context, transform: &Mat4) {
		gl.bind_buffer(UNIFORM_BUFFER, Some(self.ubo));
		gl.buffer_sub_data_u8_slice(UNIFORM_BUFFER, mem::size_of::<Mat4>() as i32, bytemuck::bytes_of(&transform.to_cols_array())); 
		gl.bind_buffer(UNIFORM_BUFFER, None);
	}

	pub unsafe fn resize(&self, gl: &Context, w: f32, h: f32) {
		let projection = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);
		gl.bind_buffer(UNIFORM_BUFFER, Some(self.ubo));
		gl.buffer_sub_data_u8_slice(UNIFORM_BUFFER, 0, bytemuck::bytes_of(&projection.to_cols_array()));
		gl.bind_buffer(UNIFORM_BUFFER, None);
	}

	pub unsafe fn drop(&self, gl: &Context) {
		gl.delete_buffer(self.ubo);
	}
}

pub struct ShaderProgram {
	pub program: NativeProgram,
}

impl ShaderProgram {
	pub unsafe fn new(gl: &Context, vsp: String, fsp: String) -> Self {
		let vert_source = fs::read_to_string(vsp).unwrap();
		let frag_soruce = fs::read_to_string(fsp).unwrap();
		let version = "#version 430";

		let program = gl.create_program().unwrap();

		let shaders = vec![
			(VERTEX_SHADER  , vert_source),
			(FRAGMENT_SHADER, frag_soruce)
		];

		let shaders = shaders.iter().map(|(shader_type, shader_source)| {
			let shader = gl.create_shader(*shader_type).unwrap();
			gl.shader_source(shader, &format!("{}\n{}", version, shader_source));
			gl.compile_shader(shader);
			if !gl.get_shader_compile_status(shader) {
				panic!("{}", gl.get_shader_info_log(shader));
			}
			gl.attach_shader(program, shader);
			shader
		}).collect::<Vec<_>>();

		gl.link_program(program);
		if !gl.get_program_link_status(program) {
			panic!("{}", gl.get_program_info_log(program));
		}

		for shader in shaders {
			gl.detach_shader(program, shader);
			gl.delete_shader(shader);
		}

		Self { program }
	}

	pub unsafe fn uniform_block_binding(&self, gl: &Context, name: String, binding: u32) {
		let index = gl.get_uniform_block_index(self.program, &name).unwrap();
		gl.uniform_block_binding(self.program, index, binding)
	}

	pub unsafe fn drop(&self, gl: &Context) {
		gl.delete_program(self.program)
	}
}
