use std::ops::Add;

pub struct Rect<T> {
	pub x: T,
	pub y: T,
	pub w: T,
	pub h: T,
}

impl<T> Rect<T>
where
	T: Copy + PartialOrd + Add<Output = T>
{
	pub fn new(x: T, y: T, w: T, h: T) -> Self {
		Self { x, y, w, h }
	}

	pub fn contains(&self, x: T, y: T) -> bool {
		let min_x = min(self.x, self.x+self.w);
		let max_x = max(self.x, self.x+self.w);
		let min_y = min(self.y, self.y+self.h);
		let max_y = max(self.y, self.y+self.h);

		x > min_x && x < max_x && y > min_y && y < max_y
	}
}

fn min<T:PartialOrd>(a: T, b: T)-> T { if a < b { a } else { b } }
fn max<T:PartialOrd>(a: T, b: T)-> T { if a > b { a } else { b } }
