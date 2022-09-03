use std::f64::consts::FRAC_2_PI;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::view::VisibleEntities;
use bevy::window::{WindowId, WindowResized};
use bevy::{input::mouse::MouseMotion, render::camera::ScalingMode};
use bevy_cam_gizmo::CameraGizmo;
use bevy_dolly::prelude::*;

use bevy_dolly::helpers::cursor_grab::DollyCursorGrab;
use bevy_dolly::prelude::cone::Cone;
use bevy_mod_picking::{
    DefaultPickingPlugins, HoverEvent, PickableBundle, PickingCameraBundle, PickingEvent,
    SelectionEvent,
};

#[derive(Component)]
struct GizmoCamera;

#[derive(Component)]
struct MainCamera;

#[derive(SystemLabel)]
struct GizmoUpdate;

#[derive(SystemLabel)]
struct MainUpdate;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(CameraGizmo)
        //.add_plugin(DollyCursorGrab)
        .add_system(dolly_component_cam_change_detection::<GizmoCamera>.label(GizmoUpdate))
        .add_system(update_camera.after(GizmoUpdate).before(MainUpdate)) //add_dolly_component(MainCamera)
        .add_system(
            dolly_component_cam_change_detection::<MainCamera>
                .label(MainUpdate)
                .after(GizmoUpdate),
        ) //.add_dolly_component(M2Camera)
        .add_system(update_other_cam.after(MainUpdate))
        .add_state(Pan::Keys)
        .add_startup_system(setup)
        .add_system(set_camera_viewports)
        //.add_system(handle_picking_events)
        .run();
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Pan {
    Mouse,
    Keys,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(Position::new(Vec3::new(0., -5., 0.)))
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
                .with(Smooth::new_rotation(1.5))
                .with(Arm::new(Vec3::Z * 4.0))
                .build(),
        )
        .insert(GizmoCamera)
        .insert(MainCamera);

    let camera = Camera3dBundle {
        projection: PerspectiveProjection {
            fov: 0.2,
            //scale: 1.0,
            //scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        camera: Camera {
            priority: 1,
            ..Default::default()
        },
        ..default()
    };

    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("barge.glb#Scene0"),
        ..default()
    });

    commands
        .spawn_bundle(camera)
        .insert(GizmoCamera)
        .insert_bundle(PickingCameraBundle::default());

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);
}

fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut right_camera: Query<&mut Camera, With<GizmoCamera>>,
) {
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();

            let mut right_camera = right_camera.single_mut();
            right_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(
                    window.physical_width() / 2 + window.physical_width() / 3,
                    0,
                ),
                physical_size: UVec2::new(
                    window.physical_width() / 5,
                    window.physical_height() / 4,
                ),
                depth: 0.0..1.0,
            });
        }
    }
}

pub fn handle_picking_events(
    mut events: EventReader<PickingEvent>,
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    for event in events.iter() {
        let entity = match event {
            PickingEvent::Selection(SelectionEvent::JustSelected(e)) => e,
            PickingEvent::Selection(SelectionEvent::JustDeselected(e)) => e,
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => e,
            PickingEvent::Hover(HoverEvent::JustLeft(e)) => e,
            PickingEvent::Clicked(e) => e,
        };

        //println!("Entity: {:?}", entity);

        /*if mouse_button_input.pressed(MouseButton::Left) {
            commands.entity(*entity).insert(PressedKey);
        } else {
            commands.entity(*entity).remove::<PressedKey>();
        }*/
    }
}

fn update_other_cam(
    mut query: ParamSet<(Query<(&mut Transform, With<MainCamera>)>, Query<&mut Rig>)>,
) {
    let mut binding = query.p1();
    let mut a = binding.single_mut();
    let p = a.driver_mut::<Position>();
    p.position = Vec3::new(0., -5., 0.);
}

#[allow(unused_must_use)]
fn update_camera(
    keys: Res<Input<KeyCode>>,
    mut pan: ResMut<State<Pan>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(Query<(&mut Transform, With<GizmoCamera>)>, Query<&mut Rig>)>,
) {
    let mut p1 = query.p1();
    let mut rig = p1.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();
    let sensitivity = Vec2::splat(10.0);

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    if pan.current().eq(&Pan::Keys) {
        if keys.just_pressed(KeyCode::Z) {
            camera_driver.rotate_yaw_pitch(-90.0, 0.0);
        }
        if keys.just_pressed(KeyCode::X) {
            camera_driver.rotate_yaw_pitch(90.0, 0.0);
        }
    } else {
        camera_driver.rotate_yaw_pitch(
            -0.1 * delta.x * sensitivity.x,
            -0.1 * delta.y * sensitivity.y,
        );
    }

    let p = rig.driver_mut::<Position>();
    p.position = Vec3::new(0., 0., 0.);

    if keys.just_pressed(KeyCode::E) {
        let result = if pan.current().eq(&Pan::Keys) {
            Pan::Mouse
        } else {
            Pan::Keys
        };
        pan.overwrite_set(result);
        println!("State:{:?}", result);
    }
}
