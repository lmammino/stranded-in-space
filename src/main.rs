use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
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
struct WrapAround;

#[derive(Component)]
struct FuelCell;

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

fn apply_velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Fuel), With<Player>>,
) {
    let (mut velocity, mut fuel) = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        velocity.x -= 10.0;
        fuel.0 -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        velocity.x += 10.0;
        fuel.0 -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        velocity.y -= 10.0;
        fuel.0 -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        velocity.y += 10.0;
        fuel.0 -= 1.0;
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

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(move_player_system)
        .add_system(apply_velocity_system)
        .add_system(wrap_around_system)
        .add_system(update_fuel_text_system)
        .run();
}
