use std::ops::{Add, Div};

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

#[derive(Component, Deref, DerefMut)]
struct BoundingBox(Vec2);

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
        .insert(Velocity(Vec2::new(
            rand::random::<f32>() * 200.0 - 100.0,
            rand::random::<f32>() * 200.0 - 100.0,
        )))
        .insert(WrapAround {})
        .insert(Fuel(100.0))
        .insert(BoundingBox(Vec2::new(30.0, 30.0)));

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
                rand::random::<f32>() * 60.0 - 30.0,
                rand::random::<f32>() * 60.0 - 30.0,
            )))
            .insert(FuelCell {})
            .insert(WrapAround {})
            .insert(Rigid {})
            .insert(BoundingBox(Vec2::new(10.0, 10.0)));
    }
}

fn rigid_collision_system(
    mut query: Query<(&mut Transform, &mut Velocity, &BoundingBox), With<Rigid>>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(t1, mut v1, bb1), (t2, mut v2, bb2)]) = iter.fetch_next() {
        let c = collide_aabb::collide(t1.translation, bb1.0, t2.translation, bb2.0);
        if c.is_some() {
            let temp = **v1;
            let vv1 = v1.as_mut();
            *vv1 = Velocity(vv1.add(v2.0).div(2.0));

            let vv2 = v2.as_mut();
            *vv2 = Velocity(vv2.add(temp).div(2.0));
        }
    }
}

fn pickup_fuel_cell_system(
    query: Query<(Entity, &Transform, &BoundingBox), With<FuelCell>>,
    mut player: Query<(&Transform, &BoundingBox, &mut Fuel), With<Player>>,
    mut commands: Commands,
) {
    let (player_transform, player_bb, mut fuel) = player.get_single_mut().unwrap();
    for (cell, cell_transform, cell_bb) in query.iter() {
        if collide_aabb::collide(
            player_transform.translation,
            player_bb.0,
            cell_transform.translation,
            cell_bb.0,
        )
        .is_some()
        {
            commands.entity(cell).despawn();
            fuel.0 += 100.0;
        }
    }
}

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
        .add_system(rigid_collision_system)
        .add_system(pickup_fuel_cell_system)
        .run();
}
