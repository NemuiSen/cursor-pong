use raylib::prelude::*;
use bevy_ecs::prelude::*;

pub struct PlayerSize(pub i32, pub i32);
#[derive(Component)]
pub struct Player(Vector2, KeyboardKey, KeyboardKey);

pub fn player_setup(
	mut commands: Commands,
	raylib: NonSendMut<(RaylibHandle, RaylibThread)>,
) {
	use raylib::consts::KeyboardKey::*;
	let (rl, _) = raylib.into_inner();
	let w = rl.get_screen_width();
	let h = rl.get_screen_height();
	commands.spawn().insert(Player(Vector2::new(
		w as f32 * 0.75,
		h as f32 / 2.0
	), KEY_UP, KEY_DOWN));
	commands.spawn().insert(Player(Vector2::new(
		w as f32 * 0.25,
		h as f32 / 2.0
	), KEY_W, KEY_S));
}

pub fn player_update(
	raylib: NonSendMut<(RaylibHandle, RaylibThread)>,
	mut players_query: Query<&mut Player>,
) {
	let (rl, _) = raylib.into_inner();
	for player in players_query.iter_mut() {
		let Player(Vector2 { y, .. }, up, down) = player.into_inner();
		if rl.is_key_down(*up  ) { *y -= 1.0; }
		if rl.is_key_down(*down) { *y += 1.0; }
	}
}

pub fn player_draw(
	player_size: Res<PlayerSize>,
	players_query: Query<&Player>,
) {
	let &PlayerSize(w, h) = player_size.into_inner();
	for &Player(Vector2 { x, y }, ..) in players_query.iter() {
		unsafe {
			ffi::DrawRectangle(
				x as i32 - w/2,
				y as i32 - h/2,
				w, h,
				Color::MAGENTA.into()
			)
		}
	}
}

