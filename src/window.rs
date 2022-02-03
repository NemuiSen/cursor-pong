use bevy_ecs::prelude::*;
use glam::Mat4;
use glow::*;
use glutin::{
	ContextBuilder,
	ContextWrapper,
	PossiblyCurrent,
	
	dpi::LogicalSize,
	event::{Event, KeyboardInput, WindowEvent, VirtualKeyCode, ElementState},
	event_loop::{ControlFlow, EventLoop},
	window::{WindowBuilder, Window},
};

pub trait IntoBytes {
	unsafe fn into_bytes(&self) -> &[u8];
}

impl IntoBytes for Mat4 {
	unsafe fn into_bytes(&self) -> &[u8] {
		let data = self.to_cols_array();
		std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<f32>())
	}
}

fn setup(
	window: NonSend<ContextWrapper<PossiblyCurrent, Window>>,
	gl: Res<Context>,
) {
	unsafe {
		gl.clear_color(0.996, 0.419, 0.039, 0.5);
		let ubo_block = gl.create_buffer().unwrap();
		let size = window.window().inner_size();
		let w = size.width  as f32;
		let h = size.height as f32;
		let proj = Mat4::orthographic_rh(
			-w,
			 w,
			-h,
			 h,
			0.0,
			1.0
		);
		gl.bind_buffer(UNIFORM_BUFFER, Some(ubo_block));
		gl.buffer_data_u8_slice(
			UNIFORM_BUFFER,
			proj.into_bytes(),
			STATIC_DRAW
		);
		gl.bind_buffer(UNIFORM_BUFFER, None);
	}
}

unsafe fn run(
	startup: SystemSet,
	update : SystemSet,
	destroy: SystemSet
) {
	let mut world = World::new();
	let mut schedule = Schedule::default();

	let el = EventLoop::new();
	let wb = WindowBuilder::new()
		.with_title("pong")
		.with_transparent(true)
		.with_inner_size(LogicalSize::new(800.0, 600.0));
	let window = ContextBuilder::new()
		.with_vsync(true)
		.build_windowed(wb, &el)
		.unwrap()
		.make_current()
		.unwrap();
	let gl = Context::from_loader_function(|s| window.get_proc_address(s) as *const _);	

	world.insert_non_send(window);
	world.insert_resource(gl);

	schedule.add_system_set_to_stage("startup", startup);
	schedule.add_system_set_to_stage("update" , update );
	schedule.add_system_set_to_stage("destroy", destroy);

	schedule.add_system_to_stage("startup", setup);
	let stage = schedule.get_stage_mut::<SystemStage>(&"startup").unwrap();
	stage.run(&mut world);

	el.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::LoopDestroyed => return,
			Event::MainEventsCleared => {
				world
					.get_non_send_resource::<ContextWrapper<PossiblyCurrent, Window>>()
					.unwrap()
					.window()
					.request_redraw();
			}
			Event::RedrawRequested(_) => {
				let gl = world.get_resource_mut::<Context>().unwrap();
				gl.clear(COLOR_BUFFER_BIT);
				let stage = schedule.get_stage_mut::<SystemStage>(&"render").unwrap();
				stage.run(&mut world);
				let window = world.get_non_send_resource_mut::<ContextWrapper<PossiblyCurrent, Window>>().unwrap();
				window.swap_buffers().unwrap();
			}
			Event::WindowEvent{ ref event, .. } => match event {
				WindowEvent::Resized(physical_size) => {
					let gl = world.get_resource::<Context>().unwrap();
					let window = world.get_non_send_resource::<ContextWrapper<PossiblyCurrent, Window>>().unwrap();
					gl.viewport(
						0,
						0,
						physical_size.width  as i32,
						physical_size.height as i32
					);
					window.resize(*physical_size);
				}
				WindowEvent::KeyboardInput {
					input: KeyboardInput {
						state: ElementState::Pressed,
						virtual_keycode: Some(VirtualKeyCode::Escape),
						..
					},
					..
				} => *control_flow = ControlFlow::Exit,
				WindowEvent::CloseRequested => {
					schedule.get_stage_mut::<SystemStage>(&"destroy").unwrap().run(&mut world);
				},
				_ => (),
			}
			_ => (),
		}
	});
}
