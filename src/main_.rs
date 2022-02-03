mod window;

use std::fs;

use glam::*;
use glow::*;
use glutin::{
	event_loop::*,
	event::*
};
use window::IntoBytes;


struct Camera {
	ubo: Buffer,
	projection: Mat4,
	view: Mat4,
}

impl Camera {
	unsafe fn new(gl: &Context, w: f32, h: f32) -> Self {
		let ubo = gl.create_buffer().unwrap();
		gl.bind_buffer(UNIFORM_BUFFER, Some(ubo));
		let projection = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);
		let view = Mat4::IDENTITY * Mat4::from_scale(vec3(200.0, 200.0, 0.0));
		gl.buffer_data_u8_slice(
			UNIFORM_BUFFER,
			&[projection.into_bytes(), view.into_bytes()].concat(),
			STATIC_DRAW
		);
		gl.bind_buffer(UNIFORM_BUFFER, None);
		gl.bind_buffer_base(UNIFORM_BUFFER, 0, Some(ubo));
		Self {
			ubo,
			projection,
			view,
		}
	}

	unsafe fn update_projection(&self, gl: &Context, w: f32, h: f32) {
		let projection = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);
		gl.bind_buffer(UNIFORM_BUFFER, Some(self.ubo));
		gl.buffer_sub_data_u8_slice(UNIFORM_BUFFER, 0, projection.into_bytes());
		gl.bind_buffer(UNIFORM_BUFFER, None);
	}
}

struct Shader {
	pub program: NativeProgram,
}

impl Shader {
	unsafe fn new(gl: &Context, vsp: String, fsp: String) -> Self {
		let vert_source = fs::read_to_string(vsp).unwrap();
		let frag_soruce = fs::read_to_string(fsp).unwrap();
		let version = "#version 410";

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

	unsafe fn uniform_block_binding(&self, gl: &Context, name: String, binding: u32) {
		let index = gl.get_uniform_block_index(self.program, &name).unwrap();
		gl.uniform_block_binding(self.program, index, binding)
	}
}

fn main() {
	unsafe {
		let event_loop = glutin::event_loop::EventLoop::new();
		let window_buider = glutin::window::WindowBuilder::new()
			.with_title("test")
			.with_transparent(true)
			.with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
		let window = glutin::ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window_buider, &event_loop)
			.unwrap()
			.make_current()
			.unwrap();
		let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

		//============[Shaders]===========//=
		let vertex_array = gl.create_vertex_array().unwrap();
		gl.bind_vertex_array(Some(vertex_array));

		let shader = Shader::new(
			&gl,
			"shaders/quad.vert".to_string(),
			"shaders/color.frag".to_string()
		);
		shader.uniform_block_binding(&gl, "camera".to_string(), 1);

		gl.use_program(Some(shader.program));

		//============[Matrix]===========//
		let size = window.window().inner_size();
		let w = size.width  as f32;
		let h = size.height as f32;
		let camera = Camera::new(&gl, w, h);

		let mut transform = Mat4::IDENTITY;
		transform *= Mat4::from_scale(vec3(1.0, 1.0, 0.0));

		let transform_unifrom = gl.get_uniform_location(shader.program, "transform").unwrap();
		gl.uniform_matrix_4_f32_slice(Some(&transform_unifrom), false, transform.to_cols_array().as_slice());

		gl.clear_color(0.996, 0.419, 0.039, 0.5);

		let clock = std::time::Instant::now();

		event_loop.run(move |event, _, control_flow| {
			let value = clock.elapsed().as_secs_f32().sin();
			let mut transform = Mat4::IDENTITY;
			transform *= Mat4::from_translation(vec3(value, value, 0.0));
			transform *= Mat4::from_rotation_z(value*std::f32::consts::PI*2.0/-1.0);
			gl.uniform_matrix_4_f32_slice(Some(&transform_unifrom), false, transform .to_cols_array().as_slice());

			*control_flow = ControlFlow::Poll;
			match event {
				Event::LoopDestroyed => {
					return;
				}
				Event::MainEventsCleared => {
					window.window().request_redraw();
				}
				Event::RedrawRequested(_) => {
					gl.clear(COLOR_BUFFER_BIT);
					gl.draw_arrays(TRIANGLE_STRIP, 0, 4);
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
						camera.update_projection(&gl, w, h);
						window.resize(*physical_size);
					},
					WindowEvent::KeyboardInput {
						input: KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::Escape),
							state: ElementState::Pressed,
							..
						},
						..
					} => *control_flow = ControlFlow::Exit,
					WindowEvent::CloseRequested => {
						gl.delete_program(shader.program);
						gl.delete_vertex_array(vertex_array);
						*control_flow = ControlFlow::Exit
					},
					_ => (),
				}
				_ => (),
			}
		});
	}
}

