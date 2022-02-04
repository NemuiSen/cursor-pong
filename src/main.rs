mod opengl;
use opengl::*;

use std::time::Instant;

use glam::*;
use glow::*;
use glutin::{
	event_loop::*,
	event::*
};

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


		//====Camera====///////////////////////////////////////
		let camera = Camera::new(&gl);
		let mut view = Mat4::IDENTITY;
		//view *= Mat4::from_translation(vec3(100.0, 70.0, 0.0));
		view *= Mat4::from_scale(vec3(200.0, 200.0, 0.0));
		camera.view(&gl, &view);


		//====Shader====///////////////////////////////////////////
		let shader = ShaderProgram::new(
			&gl,
			"shaders/quad.vert".to_string(),
			"shaders/color.frag".to_string()
		);
		shader.uniform_block_binding(&gl, "Camera".to_string(), 0);
		gl.use_program(Some(shader.program));
		let vertex_array = gl.create_vertex_array().unwrap();
		gl.bind_vertex_array(Some(vertex_array));
		let mut transform = Mat4::IDENTITY;
		let transform_location = gl.get_uniform_location(shader.program, "transform").unwrap();


		//====EventLoop====/////////////////////////////////////////
		let mut clock = Instant::now();
		gl.clear_color(0.996, 0.419, 0.039, 0.5);
		el.run(move |event, _, control_flow| {
			transform *= Mat4::from_rotation_z(clock.elapsed().as_secs_f32());
			gl.uniform_matrix_4_f32_slice(Some(&transform_location), false, &transform.to_cols_array());
			clock = Instant::now();
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
