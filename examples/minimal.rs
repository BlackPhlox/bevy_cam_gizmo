use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            vsync: false, // Disabled for this demo to remove vsync as a source of input latency
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

}
