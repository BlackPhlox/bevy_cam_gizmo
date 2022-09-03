use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_mod_picking::PickableBundle;

pub struct CameraGizmo;
impl Plugin for CameraGizmo {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera_gizmo);
    }
}

fn setup_camera_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cone_mesh = meshes.add(Mesh::from(Cone {
        height: 0.3,
        radius: 0.1,
        subdivisions: 32,
    }));

    let x_pos_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
        unlit: true,
        ..default()
    });

    let x_neg_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.4, 0.4, 0.5),
        unlit: true,
        ..default()
    });

    let y_pos_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.0, 1.0, 0.0, 0.5),
        unlit: true,
        ..default()
    });

    let y_neg_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.4, 1.0, 0.4, 0.5),
        unlit: true,
        ..default()
    });

    let z_pos_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.0, 0.0, 1.0, 0.5),
        unlit: true,
        ..default()
    });

    let z_neg_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.4, 0.4, 1.0, 0.5),
        unlit: true,
        ..default()
    });

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(0., -5., 0.),
            ..default()
        }))
        .with_children(|cell| {
            // +X
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: x_pos_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    translation: Vec3::new(0.3, 0., 0.),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());

            // -X
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: x_neg_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                    translation: Vec3::new(-0.3, 0., 0.),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());

            // +Y
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: y_pos_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    translation: Vec3::new(0., 0.3, 0.),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());

            // -Y
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: y_neg_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_x(0.),
                    translation: Vec3::new(0., -0.3, 0.),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());

            // +Z
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: z_pos_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    translation: Vec3::new(0., 0., 0.3),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());

            // -Z
            cell.spawn_bundle(PbrBundle {
                mesh: cone_mesh.clone(),
                material: z_neg_mat.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                    translation: Vec3::new(0., 0., -0.3),
                    ..default()
                },
                ..default()
            })
            .insert_bundle(PickableBundle::default());
        });
}
