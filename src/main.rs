use std::error::Error;
use glfw::{Action, Context, Key};

use cursor_pong::{
	framework::{
		traits::Drawable,
	},
	game::{
		global::{HEIGHT, WIDTH},
		player,
		ball,
	}
};

fn main() -> Result<(), Box<dyn Error>> {
	let (mut glfw, mut window, events) = init("素晴らしい!!!")?;

	let mut cursor = unsafe { ball::Ball::new(cgmath::vec2(WIDTH/2.0, HEIGHT/2.0)) };
	let mut player1 = player::Player::new( 0.7, -0.4, 0.05, 0.9, player::Keys{up: glfw::Key::Up, down: glfw::Key::Down});
	let mut player2 = player::Player::new(-0.7, -0.4, 0.05, 0.9, player::Keys{up: glfw::Key::W , down: glfw::Key::S   });

	unsafe { gl::ClearColor(0.0, 0.0, 0.0, 0.0); }
	let mut render_clock = std::time::Instant::now();
	while !window.should_close() {
		process_events(&mut window, &events);

		if render_clock.elapsed().as_secs_f32() > 1.0/60.0 {
			render_clock = std::time::Instant::now();

			unsafe {
				gl::Clear(gl::COLOR_BUFFER_BIT);
				player1.draw();
				player2.draw();
			}

			cursor.update(&mut window);
			player1.update(&window);
			player2.update(&window);

			player1.if_contains(cursor.map_coords(), || { cursor.flip_x(); });
			player2.if_contains(cursor.map_coords(), || { cursor.flip_x(); });

			window.swap_buffers();
			glfw.poll_events();
		}
	}

	Ok(())
}

use std::sync::mpsc::Receiver;
use glfw::{Glfw, Window, WindowEvent};
fn init(title: &str) -> Result<(Glfw, Window, Receiver<(f64, WindowEvent)>), Box<dyn Error>> {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
	glfw.window_hint(glfw::WindowHint::Decorated(false));
	glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

	let (mut window, events) = unsafe {
		let mode = &*glfw::ffi::glfwGetVideoMode(glfw::ffi::glfwGetPrimaryMonitor());
		WIDTH  = mode.width  as f32 - 1.0;
		HEIGHT = mode.height as f32 - 1.0;

		glfw.create_window(
			WIDTH as u32,
			HEIGHT as u32,
			title,
			glfw::WindowMode::Windowed
		).ok_or("Fail on create glfw window")?
	};

	window.make_current();
	window.set_key_polling(true);
	window.set_cursor_pos_polling(true);
	window.set_framebuffer_size_polling(true);

	gl::load_with(|s| window.get_proc_address(s) as *const _);

	Ok((glfw, window, events))
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
	for (_, event) in glfw::flush_messages(events) {
		match event {
			glfw::WindowEvent::FramebufferSize(width, height) => {
				// make sure the viewport matches the new window dimensions; note that width and
				// height will be significantly larger than specified on retina displays.
				unsafe {
					WIDTH  = width  as f32;
					HEIGHT = height as f32;
					gl::Viewport(0, 0, width, height);
				}
			}
			glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {window.set_should_close(true)},
			_ => {}
		}
	}
}
