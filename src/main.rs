mod opengl; use opengl::*;

use std::{fs, collections::HashMap, time::Instant};

use gl::types::*;
use glam::*;
use glutin::{
	event::*,
	event_loop::*,
	window::*, ContextBuilder,
};

#[derive(Default)]
struct Input {
	keys: HashMap<VirtualKeyCode, bool>,
}

impl Input {
	fn pressed(&self, key: VirtualKeyCode) -> bool {
		*self.keys.get(&key).unwrap_or(&false)
	}

	fn update<T>(&mut self, event: &Event<T>) {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput {
					input: KeyboardInput {
						state,
						virtual_keycode: Some(key),
						..
					},
					..
				} => {
					match state {
						ElementState::Pressed  => self.keys.insert(*key, true ),
						ElementState::Released => self.keys.insert(*key, false),
					}.unwrap_or_default();
				},
				_ => (),
			},
			_ => (),
		}
	}
}

struct Player {
	#[allow(dead_code)]
	shader: Shader,
	vao: GLuint,

	pub transform: Mat4,
}

impl Player {
	pub unsafe fn new() -> Self {
		let shader = Shader::new(
			fs::read_to_string("shaders/quad.vert" ).unwrap(),
			fs::read_to_string("shaders/color.frag").unwrap()
		);

		gl::UseProgram(shader.program);
		let mut vao = 0;
		gl::GenVertexArrays(1, &mut vao);

		Self {
			shader,
			vao,
			transform: Mat4::IDENTITY,
		}
	}

	pub unsafe fn update_transform(&self) {
		let name = std::ffi::CString::new("transform").unwrap();
		let transform_location = gl::GetUniformLocation(self.shader.program, name.as_ptr());
		gl::UseProgram(self.shader.program);
		gl::UniformMatrix4fv(
			transform_location,
			1,
			gl::FALSE,
			self.transform.to_cols_array().as_ptr()
		);
	}

	pub unsafe fn draw(&self) {
		gl::UseProgram(self.shader.program);
		gl::BindVertexArray(self.vao);
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
	}
}

impl Drop for Player {
	fn drop(&mut self) {
	    unsafe {
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}
}





fn main() {
	unsafe {
		let el = EventLoop::new();
		let wb = WindowBuilder::new()
			.with_title("pong!")
			.with_transparent(true);
		let context = ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(wb, &el)
			.unwrap()
			.make_current()
			.unwrap();
		gl::load_with(|s| context.get_proc_address(s));

		let camera = Camera::new();

		let mut player1 = Player::new();
		let mut player2 = Player::new();
		player1.transform *= Mat4::from_translation(vec3(-0.5, 0.0, 0.0)) * Mat4::from_scale(vec3(1.0/10.0, 1.0/2.0, 0.0));
		player2.transform *= Mat4::from_translation(vec3( 0.5, 0.0, 0.0)) * Mat4::from_scale(vec3(1.0/10.0, 1.0/2.0, 0.0));
		player1.update_transform();
		player2.update_transform();

		gl::ClearColor(0.996, 0.419, 0.039, 0.5);
		let mut clock = Instant::now();
		let mut input = Input::default();
		el.run(move |event, _, control_flow| {
			let mut delta = Vec2::ZERO;
			if input.pressed(VirtualKeyCode::W) { delta.y += 1.0; }
			if input.pressed(VirtualKeyCode::A) { delta.x -= 1.0; }
			if input.pressed(VirtualKeyCode::S) { delta.y -= 1.0; }
			if input.pressed(VirtualKeyCode::D) { delta.x += 1.0; }
			let delta = delta.normalize_or_zero().extend(0.0);
			player1.transform *= Mat4::from_translation(delta * clock.elapsed().as_secs_f32() * 5.0);
			player1.update_transform();

			clock = Instant::now();


			*control_flow = ControlFlow::Poll;
			input.update(&event);
			match event {
				Event::LoopDestroyed => {
					return;
				}
				Event::MainEventsCleared => {
					context.window().request_redraw();
				}
				Event::RedrawRequested(_) => {
					gl::Clear(gl::COLOR_BUFFER_BIT);
					player1.draw();
					player2.draw();
					context.swap_buffers().unwrap();
				}
				Event::WindowEvent{ ref event, .. } => match event {
					WindowEvent::Resized(physical_size) => {
						gl::Viewport(
							0,
							0,
							physical_size.width  as i32,
							physical_size.height as i32,
						);
						let w = physical_size.width  as f32;
						let h = physical_size.height as f32;
						let sz = w.min(h) as f32;
						camera.view(Mat4::from_scale(vec3(sz, sz, 0.0)));
						camera.resize(w, h);
						context.resize(*physical_size);
					},
					WindowEvent::KeyboardInput {
						input: KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::Escape),
							state: ElementState::Pressed,
							..
						},
						..
					} => {
						*control_flow = ControlFlow::Exit;
					}
					WindowEvent::CloseRequested => {
						*control_flow = ControlFlow::Exit
					},
					_ => (),
				}
				_ => (),
			}
		});
	}
}
