use raylib::prelude::*;
use bevy_ecs::prelude::*;

pub struct PlayerSize(pub i32, pub i32);
#[derive(Component)]
pub struct Player(pub Vector2, pub KeyboardKey, pub KeyboardKey);

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
		if rl.is_key_down(*up  ) { *y -= 10.0; }
		if rl.is_key_down(*down) { *y += 10.0; }
	}
}

