use bevy_ecs::{prelude::*, schedule::ShouldRun};
use raylib::prelude::*;

fn main() {
	let (rl, thread) = raylib::init()
		.size(640, 480)
		.title("Hello, World")
		.transparent()
		.build();

	let mut world = World::new();
	world.insert_non_send((rl, thread));
	world.insert_resource(PlayerSize(20, 100));
	let mut schelude = Schedule::default();
	let mut startup = SystemStage::parallel();
	startup
		.add_system(setup);
	schelude.add_stage("startup", startup);
	let mut update = SystemStage::parallel();
	update
		.add_system(render)
		.add_system(player)
		.set_run_criteria(should_close);
	schelude.add_stage_after("startup", "update", update);
	schelude.run(&mut world);
}

struct PlayerSize(i32, i32);

fn setup(
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

fn should_close(raylib: NonSendMut<(RaylibHandle, RaylibThread)>) -> ShouldRun {
	let (rl, _) = raylib.into_inner();
	match rl.window_should_close() {
		true  => ShouldRun::No ,
		false => ShouldRun::YesAndCheckAgain,
	}
}

fn render(
	raylib: NonSendMut<(RaylibHandle, RaylibThread)>,
	player_size: Res<PlayerSize>,
	players_query: Query<&Player>,
) {
	let (rl, thread) = raylib.into_inner();
	let &PlayerSize(w, h) = player_size.into_inner();
	let mut d = rl.begin_drawing(&thread);
	d.clear_background(Color::BLANK);
	d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
	for &Player(Vector2 { x, y }, ..) in players_query.iter() {
		d.draw_rectangle(x as i32, y as i32, w, h, Color::MAGENTA);
	}
}

fn player(
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

#[derive(Component)]
struct Player(Vector2, KeyboardKey, KeyboardKey);