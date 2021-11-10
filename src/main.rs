#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";
const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Materials {
    player_materials: Handle<ColorMaterial>,
}
struct WinSize {
    w: f32,
    h: f32,
}

struct Player;
struct PlayerSpeed(f32);
impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(250.0)
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Shooter".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        w: window.height(),
        h: window.height(),
    })
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    //spawn sprite
    let bottom = -win_size.h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerSpeed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let x_dir = if keyboard_input.pressed(KeyCode::A) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::D) {
            1.0
        } else {
            0.0
        };

        let y_dir = if keyboard_input.pressed(KeyCode::W) {
            1.0
        } else if keyboard_input.pressed(KeyCode::S) {
            -1.0
        } else {
            0.0
        };
        
        transform.translation.x += x_dir * speed.0 * TIME_STEP;
        transform.translation.y += y_dir * speed.0 * TIME_STEP;
    }
}