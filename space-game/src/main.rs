use bevy::{color::palettes::basic::PURPLE, prelude::*};

#[derive(Component)]
struct Ship;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    meshes.add(Rectangle::new(50.0, 100.0));
    MeshMaterial2d(materials.add(Color::from(PURPLE)));
}
fn steer_ship( keyboard_input: Res<ButtonInput<KeyCode>>, mut ship_transform: Single<&mut Transform, With<Ship>>,){


}
fn move_ship(mut ship_transform: Single<&mut Transform, With<Ship>>,){


}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (steer_ship,move_ship).chain())
        .run();
}
