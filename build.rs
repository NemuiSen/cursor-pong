#[cfg(windows)]
fn main() {
	winres::WindowsResource::new()
		.set_icon("pong_icon.ico")
		.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}