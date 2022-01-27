use bevy_ecs::{prelude::*, schedule::ShouldRun};
use raylib::prelude::*;

mod player; use player::*;

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(640, 480)
		.title("Hello, World")
		.transparent()
		.build();
	rl.set_target_fps(60);
	rl.set_mouse_scale(1., 1.);

	let mut world = World::new();
	world.insert_non_send((rl, thread));
	world.insert_resource(PlayerSize(20, 100));
	let mut schelude = Schedule::default();
	let mut startup = SystemStage::parallel();
	startup
		.add_system(player_setup);
	schelude.add_stage("startup", startup);
	let mut update = SystemStage::parallel();
	update
		.add_system(player_update.label("update"))
		.add_system(render.label("render").after("update"))
		.add_system(player_draw.after("render"))
		.set_run_criteria(should_close);
	schelude.add_stage_after("startup", "update", update);
	schelude.run(&mut world);
}

fn should_close(raylib: NonSendMut<(RaylibHandle, RaylibThread)>) -> ShouldRun {
	let (rl, _) = raylib.into_inner();
	match rl.window_should_close() {
		true  => ShouldRun::No ,
		false => ShouldRun::YesAndCheckAgain,
	}
}

fn render() {
	unsafe {
		ffi::EndDrawing();
		ffi::BeginDrawing();
		ffi::ClearBackground(Color::BLANK.into());
	}
}

