pub trait Drawable {
	unsafe fn draw(&self);
}

pub trait Transformable {
	fn transform(&mut self, value: *const f32);
}
