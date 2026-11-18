use bevy::{color::palettes::basic::PURPLE, math::EulerRot::ZXY, prelude::*};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Laser;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 100.0))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Ship,
        Player,
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
fn steer_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ship_transform: Single<(&Transform, &mut Velocity), With<Player>>,
) {
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction_x += 5.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction_x -= 2.5;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction_y += 0.1;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction_y -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        let clamped_velocity = ship_transform
            .1
            .0
            //.clamp(vec3(-5.0, -5.0, -0.1), vec3(5.0, 5.0, 0.1));
            .clamp(vec3(0.0, 0.0, -0.1), vec3(0.0, 0.0, 0.1));
        ship_transform.1.0 -= clamped_velocity;
    }
    ship_transform.1.0.y += direction_x * (ship_transform.0.rotation.to_euler(ZXY).0).cos();
    ship_transform.1.0.x += direction_x * -(ship_transform.0.rotation.to_euler(ZXY).0).sin();
    ship_transform.1.0.z += direction_y;
}
fn move_ships(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
        transform.rotate_z(velocity.0.z * time.delta_secs());
    }
}

fn shoot_laser(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ship_transform: Single<&Transform, With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyF) {
        return;
    }

    let forward = ship_transform.rotation * Vec3::Y;
    let laser_position = ship_transform.translation + forward * 65.0;

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10.0, 30.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.1, 0.1))),
        Laser,
        Velocity(forward * 500.0),
        Transform::from_translation(laser_position).with_rotation(ship_transform.rotation),
    ));
}

fn remove_distant_lasers(mut commands: Commands, lasers: Query<(Entity, &Transform), With<Laser>>) {
    for (entity, transform) in &lasers {
        if transform.translation.x.abs() > 1000.0 || transform.translation.y.abs() > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (steer_ship, shoot_laser, move_ships, remove_distant_lasers).chain(),
        )
        .run();
}
