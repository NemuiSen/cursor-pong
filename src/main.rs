use glow::{HasContext, COLOR_BUFFER_BIT, VERTEX_SHADER, TRIANGLES, FRAGMENT_SHADER};
use glutin::{event_loop::ControlFlow, event::{Event, WindowEvent}};

fn main() {
	unsafe {
		let event_loop = glutin::event_loop::EventLoop::new();
		let window_buider = glutin::window::WindowBuilder::new()
			.with_title("test")
			.with_transparent(true)
			.with_inner_size(glutin::dpi::LogicalSize::new(500.0, 500.0));
		let window = glutin::ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window_buider, &event_loop)
			.unwrap()
			.make_current()
			.unwrap();
		let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

		let vertex_array = gl.create_vertex_array().unwrap();
		gl.bind_vertex_array(Some(vertex_array));

		let (vs_source, fs_source) = (
			r#"const vec2 verts[3] = vec2[3](
				vec2(0.5f, 1.0f),
				vec2(0.0f, 0.0f),
				vec2(1.0f, 0.0f)
			);
			out vec2 vert;
			void main() {
				vert = verts[gl_VertexID];
				gl_Position = vec4(vert - 0.5, 0.0, 1.0);
			}"#,
			r#"precision mediump float;
			in vec2 vert;
			out vec4 color;
			void main() {
				color = vec4(vert, 0.5, 1.0);
			}"#,
		);
		let version = "#version 410";
		let program = gl.create_program().unwrap();
		
		let vshader = gl.create_shader(VERTEX_SHADER).unwrap();
		gl.shader_source(vshader, &format!("{}\n{}", version, vs_source));
		gl.compile_shader(vshader);
		if !gl.get_shader_compile_status(vshader) {
			panic!("{}", gl.get_shader_info_log(vshader));
		}
		gl.attach_shader(program, vshader);
		
		let fshader = gl.create_shader(FRAGMENT_SHADER).unwrap();
		gl.shader_source(fshader, &format!("{}\n{}", version, fs_source));
		gl.compile_shader(fshader);
		if !gl.get_shader_compile_status(fshader) {
			panic!("{}", gl.get_shader_info_log(fshader));
		}
		gl.attach_shader(program, fshader);

		gl.link_program(program);
		if !gl.get_program_link_status(program) {
			panic!("{}", gl.get_program_info_log(program));
		}

		gl.detach_shader(program, vshader);
		gl.detach_shader(program, fshader);
		gl.delete_shader(vshader);
		gl.delete_shader(fshader);

		gl.use_program(Some(program));
		gl.clear_color(0.996, 0.419, 0.039, 0.5);

		event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;
			match event {
				Event::LoopDestroyed => {
					return;
				}
				Event::MainEventsCleared => {
					window.window().request_redraw();
				}
				Event::RedrawRequested(_) => {
					gl.clear(COLOR_BUFFER_BIT);
					gl.draw_arrays(TRIANGLES, 0, 3);
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
						window.resize(*physical_size);
					},
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

