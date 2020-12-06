use crate::global::{BALL_SPEED, WIDTH, HEIGHT};

pub struct CursorBall {
	pos: cgmath::Vector2<f32>,
	dir: cgmath::Vector2<f32>,
}

#[allow(dead_code)]
impl CursorBall {
	pub fn new(pos: cgmath::Vector2<f32>) -> Self {
		Self {
			pos,
			dir: cgmath::vec2(1.0, 1.0),
		}
	}

	pub fn update(&mut self, window: &mut glfw::Window) {
		self.pos.x += self.dir.x * BALL_SPEED;
		self.pos.y += self.dir.y * BALL_SPEED;
		unsafe {
			if self.pos.x >= WIDTH || self.pos.x <= 0.0 {
				self.flip_x();
			}

			if self.pos.y >= HEIGHT || self.pos.y <= 0.0 {
				self.flip_y();
			}

			window.set_cursor_pos(
				self.pos.x as f64,
				self.pos.y as f64
			);
		}
	}

	pub fn flip_x(&mut self) { self.dir.x = -self.dir.x; }
	pub fn flip_y(&mut self) { self.dir.y = -self.dir.y; }

	pub fn map_coords(&self) -> cgmath::Vector2<f32> {
		unsafe {
			cgmath::vec2(
				  self.pos.x / WIDTH  * 2.0 - 1.0,
				-(self.pos.y / HEIGHT * 2.0 - 1.0)
			)
		}
	}
}
