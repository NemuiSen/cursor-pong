mod helper;
use std::marker::PhantomData;
use gl::types::*;

pub const _VSH: &str = r#"
	#version 330 core
	layout (location = 0) in vec2 pos;
	layout (location = 1) in vec4 color;
	out vec4 ourColor;
	uniform mat4 transform;
	void main() {
		gl_Position = transform * vec4(pos, 0.0, 1.0);
		ourColor = color;
	}
"#;

pub const _FSH: &str = r#"
	#version 330 core
	in vec4 ourColor;
	out vec4 FragColor;
	void main() {
		FragColor = ourColor;
	}
"#;



#[derive(Copy, Clone)]
pub struct VertexPoint {
	pub pos:   [GLfloat; 2],
	pub color: [GLfloat; 4],
}

impl VertexPoint {
	pub fn new(pos: [GLfloat; 2], color: [GLfloat; 4]) -> Self {
		Self {pos, color}
	}
}

impl std::fmt::Debug for VertexPoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{:?}{:?}", self.pos, self.color)
	}
}



pub struct Vertex<T> {
	vbo: GLuint,
	vao: GLuint,
	len: usize,
	phantom: PhantomData<T>
}

#[allow(dead_code)]
impl<T> Vertex<T> {
	pub fn new(config: &[i32], points: &[T]) -> Self {
		unsafe {
			let (mut vbo, mut vao) = (0, 0);
			gl::GenBuffers(1, &mut vbo);
			gl::GenVertexArrays(1, &mut vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			helper::array_buffer_data(points);
			gl::BindVertexArray(vao);
			helper::config_vertex_array::<T>(config);

			Self {
				vbo,
				vao,
				len: points.len(),
				phantom: PhantomData,
			}
		}
	}

	pub fn config(&self, config: &[i32]) {
		unsafe {
			gl::BindVertexArray(self.vao);
			helper::config_vertex_array::<T>(config);
		}
	}

	pub fn set_data(&mut self, data: &[T]) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			helper::array_buffer_data(data);
		}
	}

	pub fn get_data(&self) -> &[T] {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			let data = helper::map_buffer::<T>(self.len, gl::READ_ONLY);
			gl::UnmapBuffer(gl::ARRAY_BUFFER);
			data
		}
	}
}

impl<T> crate::traits::Drawable for Vertex<T> {
	fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.len as GLsizei);
		}
	}
}

impl<T> Drop for Vertex<T> {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}
