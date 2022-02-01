use std::fs;

use glam::*;
use glow::*;
use glutin::{
	event_loop::*,
	event::*
};

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

		let vertex_array = gl.create_vertex_array().unwrap();
		gl.bind_vertex_array(Some(vertex_array));

		let vert_source = fs::read_to_string("shaders/quad.vert").unwrap();
		let frag_source = fs::read_to_string("shaders/color.frag").unwrap();

		let version = "#version 410";
		let program = gl.create_program().unwrap();

		let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
		gl.shader_source(vert_shader, &format!("{}\n{}", version, vert_source));
		gl.compile_shader(vert_shader);
		if !gl.get_shader_compile_status(vert_shader) {
			panic!("{}", gl.get_shader_info_log(vert_shader));
		}
		gl.attach_shader(program, vert_shader);

		let frag_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
		gl.shader_source(frag_shader, &format!("{}\n{}", version, frag_source));
		gl.compile_shader(frag_shader);
		if !gl.get_shader_compile_status(frag_shader) {
			panic!("{}", gl.get_shader_info_log(frag_shader));
		}
		gl.attach_shader(program, frag_shader);

		gl.link_program(program);
		if !gl.get_program_link_status(program) {
			panic!("{}", gl.get_program_info_log(program));
		}

		gl.detach_shader(program, vert_shader);
		gl.detach_shader(program, frag_shader);
		gl.delete_shader(vert_shader);
		gl.delete_shader(frag_shader);

		gl.use_program(Some(program));

		let size = window.window().inner_size();
		let w = size.width  as f32;
		let h = size.height as f32;
		let projection = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);

		let mut transform = Mat4::IDENTITY;
		//transform *= Mat4::from_translation(vec3(0.0, 9.0, 0.0));
		transform *= Mat4::from_scale(vec3(1.0, 1.0, 0.0));
		//transform *= Mat4::from_rotation_z(4.0/std::f32::consts::PI);

		let mut view = Mat4::IDENTITY;
		view *= Mat4::from_translation(vec3(0.0, 0.0, 0.0));
		view *= Mat4::from_scale(vec3(200.0, 200.0, 0.0));

		let projection_uniform = gl.get_uniform_location(program, "projection").unwrap();
		let view_uniform       = gl.get_uniform_location(program, "view"      ).unwrap();
		let transform_unifrom  = gl.get_uniform_location(program, "transform" ).unwrap();
		gl.uniform_matrix_4_f32_slice(Some(&projection_uniform), false, projection.to_cols_array().as_slice());
		gl.uniform_matrix_4_f32_slice(Some(&view_uniform      ), false, view      .to_cols_array().as_slice());
		gl.uniform_matrix_4_f32_slice(Some(&transform_unifrom ), false, transform .to_cols_array().as_slice());

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
						let projection = Mat4::orthographic_rh(
							-w,
							 w,
							-h,
							 h,
							0.0,
							1.0
						);
						gl.uniform_matrix_4_f32_slice(
							Some(&projection_uniform),
							false,
							projection.to_cols_array().as_slice()
						);
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
						gl.delete_program(program);
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

