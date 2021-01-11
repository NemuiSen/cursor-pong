use std::slice::from_raw_parts;
use std::mem::size_of;
use std::os::raw::c_void;
use gl::types::*;

pub unsafe fn array_buffer_data<T>(data: &[T]) {
	gl::BufferData(
		gl::ARRAY_BUFFER,
		(data.len() * size_of::<T>()) as GLsizeiptr,
		data.as_ptr() as *const c_void,
		gl::DYNAMIC_DRAW
	);
}

pub unsafe fn map_buffer<'a, T>(data_len: usize, access: GLuint) -> &'a[T] {
	let data_ptr = gl::MapBuffer(gl::ARRAY_BUFFER, access) as *mut T;
	from_raw_parts(data_ptr, data_len)
}

pub unsafe fn config_vertex_array<T>(config: &[i32]) {
	let mut offset = 0;
	for (index, &size) in config.iter().enumerate() {
		if size <= 0 { panic!("[ERROR::VERTEX_ARRAY::CONFIG]: Any value of config can't be zero or less"); }
		gl::EnableVertexAttribArray(index as GLuint);
		gl::VertexAttribPointer(
			index as GLuint,
			size,
			gl::FLOAT,
			gl::FALSE,
			std::mem::size_of::<T>() as GLsizei,
			(offset * std::mem::size_of::<GLfloat>()) as *const GLfloat as *const c_void
		);
		// println!("index = {} | size = {} | offset = {}", index, size, offset);
		offset += size as usize;
	}
}
