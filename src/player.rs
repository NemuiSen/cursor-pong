use crate::global::PLAYER_SPEED;
use crate::rect::Rect;
use crate::traits::{Drawable, Transformable};
// use crate::cursor_ball::CursorBall;
use cgmath::*;
use gl::types::*;

#[path ="vertex/mod.rs"] mod vertex;
#[path ="shader/mod.rs"] mod shader;

pub struct Keys {
	pub up: glfw::Key,
	pub down: glfw::Key,
}

fn create_rect(w: GLfloat, h: GLfloat) -> [vertex::VertexPoint; 4] {
	let color = [1.0, 1.0, 1.0, 1.0];
	[
		vertex::VertexPoint::new([-w/2.0, -h/2.0], color),
		vertex::VertexPoint::new([ w/2.0, -h/2.0], color),
		vertex::VertexPoint::new([-w/2.0,  h/2.0], color),
		vertex::VertexPoint::new([ w/2.0,  h/2.0], color),
	]
}

pub struct Player {
	vertex: vertex::Vertex<vertex::VertexPoint>,
	shader: shader::Shader,
	keys: Keys,
	rect: Rect<GLfloat>
}

#[allow(dead_code)]
impl Player {
	pub fn new(x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat, keys: Keys) -> Self {
		Self {
			vertex: vertex::Vertex::new(&[2, 4], &create_rect(w, h)),
			shader: shader::Shader::new(vertex::_VSH, vertex::_FSH),
			keys,
			rect: Rect::new(x, y, w, h)
		}
	}

	pub fn update(&mut self, window: &glfw::Window) {
		if window.get_key(self.keys.up  ) == glfw::Action::Press && self.rect.y <  1.0 { self.rect.y += PLAYER_SPEED; }
		if window.get_key(self.keys.down) == glfw::Action::Press && self.rect.y > -1.0 { self.rect.y -= PLAYER_SPEED; }

		let mut trans = cgmath::Matrix4::<GLfloat>::identity();
		trans = trans * cgmath::Matrix4::from_translation(cgmath::vec3(self.rect.x, self.rect.y, 0.0));
		self.transform(trans.as_ptr());
	}

	pub fn if_contains(&self, point: cgmath::Vector2<GLfloat>, mut func: impl FnMut()) {
		let rect = Rect::new(
			self.rect.x-self.rect.w/2.0,
			self.rect.y-self.rect.h/2.0,
			self.rect.w,
			self.rect.h,
		);
		if rect.contains(point.x, point.y) {
			func();
		}
	}
}

impl Drawable for Player {
	fn draw(&self) {
		self.shader.use_program();
		self.vertex.draw();
	}
}

impl Transformable for Player {
	fn transform(&mut self, value: *const GLfloat) {
		self.shader.set_matrix4("transform", value);
	}
}
