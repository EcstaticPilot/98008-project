use bevy::{color::palettes::basic::PURPLE, input::keyboard, math::EulerRot::ZXY, prelude::*};
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity(Vec3);

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
        direction_x -= 5.0;
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
            .clamp(vec3(-5.0, -5.0, -0.1), vec3(5.0, 5.0, 0.1));
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
        println!("{}", transform.rotation.to_euler(ZXY).0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (steer_ship, move_ships).chain())
        .run();
}
