use bevy::{prelude::*, sprite::collide_aabb};

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Fuel(f32);

#[derive(Component)]
struct FuelStatusText;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Rigid;

#[derive(Component)]
struct WrapAround;

#[derive(Component)]
struct FuelCell;

struct FuelCellSpawner(Timer);

fn setup(mut commands: Commands, mut windows: ResMut<Windows>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    window.set_title("Stranded in space".to_string());
    window.set_resolution(WIN_WIDTH, WIN_HEIGHT);
    window.set_resizable(false);

    // Spawn the player
    commands
        .spawn()
        .insert(Player {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.5, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Velocity(Vec2::new(100.0, 100.0)))
        .insert(WrapAround {})
        .insert(Fuel(100.0));

    // Spawn the fuel text
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Fuel: ",
                    TextStyle {
                        font: asset_server.load("fonts/Monocraft.otf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "100",
                    TextStyle {
                        font: asset_server.load("fonts/Monocraft.otf"),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.5, 1.0),
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(FuelStatusText);
}

fn update_fuel_text_system(
    mut query: Query<&mut Text, With<FuelStatusText>>,
    fuel_query: Query<&Fuel>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", fuel_query.single().0);
}

fn apply_velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn move_player_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Fuel), With<Player>>,
) {
    let (mut velocity, mut fuel) = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        velocity.x -= 60.0 * 10.0 * time.delta_seconds();
        fuel.0 -= 60.0 * 0.5 * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Right) {
        velocity.x += 60.0 * 10.0 * time.delta_seconds();
        fuel.0 -= 60.0 * 0.5 * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Down) {
        velocity.y -= 60.0 * 10.0 * time.delta_seconds();
        fuel.0 -= 60.0 * 0.5 * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Up) {
        velocity.y += 60.0 * 10.0 * time.delta_seconds();
        fuel.0 -= 60.0 * 0.5 * time.delta_seconds();
    }
}

fn wrap_around_system(mut query: Query<&mut Transform, With<WrapAround>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    for mut transform in query.iter_mut() {
        if transform.translation.x > width / 2.0 {
            transform.translation.x = -width / 2.0;
        }

        if transform.translation.x < -width / 2.0 {
            transform.translation.x = width / 2.0;
        }

        if transform.translation.y > height / 2.0 {
            transform.translation.y = -height / 2.0;
        }

        if transform.translation.y < -height / 2.0 {
            transform.translation.y = height / 2.0;
        }
    }
}

fn fuel_cell_spawn_system(
    time: Res<Time>,
    mut spawner: ResMut<FuelCellSpawner>,
    mut commands: Commands,
) {
    if spawner.0.tick(time.delta()).just_finished() {
        let x = rand::random::<f32>() * WIN_WIDTH - WIN_WIDTH / 2.0;
        let y = rand::random::<f32>() * WIN_HEIGHT - WIN_HEIGHT / 2.0;
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 0.0),
                    translation: Vec3::new(x, y, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.9, 0.1),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity(Vec2::new(
                rand::random::<f32>() * 40.0 - 20.0,
                rand::random::<f32>() * 40.0 - 20.0,
            )))
            .insert(FuelCell {})
            .insert(WrapAround {})
            .insert(Rigid {});
    }
}

// fn rigid_collision_system(
//     mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Rigid>>,
//     query_ref: Query<(Entity, &Transform, &Velocity), With<Rigid>>,
// ) {
//     for (id, transform, velocity) in query.iter_mut() {
//         for (id_ref, ref_transform, ref_velocity) in query_ref.iter() {
//             if id == id_ref {
//                 continue;
//             }
//             let c = collide_aabb::collide(
//                 transform.translation,
//                 Vec2::new(10.0, 10.0),
//                 ref_transform.translation,
//                 Vec2::new(10.0, 10.0),
//             );
//             if c.is_some() {
//                 println!("Collision!");
//             }
//         }
//     }
// }

fn main() {
    App::new()
        .insert_resource(FuelCellSpawner(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(move_player_system)
        .add_system(apply_velocity_system)
        .add_system(wrap_around_system)
        .add_system(update_fuel_text_system)
        .add_system(fuel_cell_spawn_system)
        // .add_system(rigid_collision_system)
        .run();
}
