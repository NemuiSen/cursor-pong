mod opengl;
use opengl::*;

use std::{time::Instant, collections::HashMap};

use glam::*;
use glow::*;
use glutin::{
	event_loop::*,
	event::*
};

#[derive(Default)]
struct Input {
	keys: HashMap<VirtualKeyCode, bool>
}

impl Input {
	pub fn update<T>(&mut self, event: &Event<T>) {
		match &event {
			Event::WindowEvent { event, .. } => {
				match event {
					WindowEvent::KeyboardInput {
						input: KeyboardInput {
							state,
							virtual_keycode: Some(key),
							..
						},
						..
					} => {
							match state {
							ElementState::Pressed  => self.keys.insert(*key, true ).unwrap_or_default(),
							ElementState::Released => self.keys.insert(*key, false).unwrap_or_default(),
						};
					},
					_ => ()
				}
			}
			_ => ()
		}
	}

	pub fn pressed(&self, key: VirtualKeyCode) -> bool {
		*self.keys.get(&key).unwrap_or(&false)
	}
}

struct Player {
	shader: ShaderProgram,
	vao: NativeVertexArray,

	pos: Vec2,
	rot: f32 ,
	scl: Vec2,
}

impl Player {
	unsafe fn new(gl: &Context) -> Self {
		let shader = ShaderProgram::new(
			&gl,
			"shaders/quad.vert".to_string(),
			"shaders/color.frag".to_string()
		);
		shader.uniform_block_binding(&gl, "Camera".to_string(), 0);
		gl.use_program(Some(shader.program));
		let vao = gl.create_vertex_array().unwrap();
		Self {
			shader,
			vao,
			pos: Vec2::splat(0.0),
			rot: 0.0,
			scl: Vec2::splat(1.0)
		}
	}

	pub unsafe fn draw(&self, gl: &Context) {
		gl.bind_vertex_array(Some(self.vao));
		gl.draw_arrays(TRIANGLE_STRIP, 0, 4);
	}

	pub fn input(&mut self, input: &Input, delta: &Instant) {
		let SPEED: f32 = 0.1;
		let mut dir = Vec2::default();
		if input.pressed(VirtualKeyCode::W) { dir.y += SPEED }
		if input.pressed(VirtualKeyCode::S) { dir.y -= SPEED }
		if input.pressed(VirtualKeyCode::D) { dir.x += SPEED }
		if input.pressed(VirtualKeyCode::A) { dir.x -= SPEED }
		dir *= delta.elapsed().as_secs_f32();
		let dir = dir.normalize_or_zero();
		self.pos += dir;
	}

	pub unsafe fn update_transform(&mut self, gl: &Context) {
		let transform = Mat4::IDENTITY * Mat4::from_scale_rotation_translation(
			self.scl.extend(0.0),
			Quat::from_rotation_z(self.rot),
			self.pos.extend(0.0)
		);
		let transform_location = gl.get_uniform_location(self.shader.program, "transform").unwrap();
		gl.uniform_matrix_4_f32_slice(Some(&transform_location), false, &transform.to_cols_array());
	}
	
	pub unsafe fn drop(&self, gl: &Context) {
		self.shader.drop(gl);
		gl.delete_vertex_array(self.vao);
	}
}

fn main() {
	unsafe {
		let el = glutin::event_loop::EventLoop::new();
		let wb = glutin::window::WindowBuilder::new()
			.with_title("test")
			.with_transparent(true)
			.with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
		let window = glutin::ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(wb, &el)
			.unwrap()
			.make_current()
			.unwrap();
		let gl = Context::from_loader_function(|s| window.get_proc_address(s) as *const _);


		let camera = Camera::new(&gl);
		let mut view = Mat4::IDENTITY;
		view *= Mat4::from_scale(vec3(200.0, 200.0, 0.0));
		camera.view(&gl, &view);	
		let mut player =  Player::new(&gl);


		//====EventLoop====/////////////////////////////////////////
		let mut clock = Instant::now();
		gl.clear_color(0.996, 0.419, 0.039, 0.5);
		let mut input = Input::default();
		el.run(move |event, _, control_flow| {
			player.rot += clock.elapsed().as_secs_f32();
			player.input(&input, &clock);
			player.update_transform(&gl);
			clock = Instant::now();
			*control_flow = ControlFlow::Poll;
			input.update(&event);
			match event {
				Event::LoopDestroyed => {
					return;
				}
				Event::MainEventsCleared => {
					window.window().request_redraw();
				}
				Event::RedrawRequested(_) => {
					gl.clear(COLOR_BUFFER_BIT);
					player.draw(&gl);
					window.swap_buffers().unwrap();
				}
				Event::WindowEvent{ ref event, .. } => match event {
					WindowEvent::Resized(physical_size) => {
						gl.viewport(
							0,
							0,
							physical_size.width  as i32,
							physical_size.height as i32,
						);
						let w = physical_size.width  as f32;
						let h = physical_size.height as f32;
						camera.resize(&gl, w, h);
						window.resize(*physical_size);
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
						camera.drop(&gl);
						player.drop(&gl);
						*control_flow = ControlFlow::Exit
					},
					_ => (),
				}
				_ => (),
			}
		});
	}
}
