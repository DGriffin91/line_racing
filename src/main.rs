// Demonstrates using ShapeCommands to spawn entity backed shapes

pub mod lines;
pub mod sampling;

use std::time::Instant;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    prelude::*,
    window::{PresentMode, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
use bevy_polyline::{
    prelude::{Polyline, PolylineBundle, PolylineMaterial},
    PolylinePlugin,
};
use bevy_vector_shapes::prelude::*;
use lines::{LineList, LineMaterial};
use sampling::hash_noise;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                resolution: WindowResolution::new(1024.0, 1024.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            ShapePlugin::default(),
            PolylinePlugin,
            MaterialPlugin::<LineMaterial>::default(),
        ))
        .add_systems(Startup, camera)
        .add_systems(Update, benchmark);

    if args.contains(&"--bevy_lines_example_retained".to_string()) {
        app.add_systems(Startup, bevy_lines_example_retained);
    }
    if args.contains(&"--bevy_plane_3d_retained".to_string()) {
        app.add_systems(Startup, bevy_plane_3d_retained);
    }
    if args.contains(&"--bevy_polyline_retained".to_string()) {
        app.add_systems(Startup, bevy_polyline_retained);
    }
    if args.contains(&"--bevy_vector_shapes_retained".to_string()) {
        app.add_systems(Startup, bevy_vector_shapes_retained);
    }
    if args.contains(&"--bevy_vector_shapes_immidate".to_string()) {
        app.add_systems(Update, bevy_vector_shapes_immidate);
    }
    if args.contains(&"--gizmos_immidate".to_string()) {
        app.add_systems(Update, gizmos_immidate);
    }
    app.run();
}

const COUNT: u32 = 100_000;

fn camera(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    for (_, config, _) in config_store.iter_mut() {
        config.line_width = 1.0;
    }
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        ..default()
    });
}

fn bevy_vector_shapes_retained(mut shapes: ShapeCommands) {
    shapes.thickness = 0.002;
    shapes.cap = Cap::None;
    shapes.disable_laa = true;

    for x in 0..COUNT {
        shapes.line(
            vec3(
                hash_noise(x, 0, 0),
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0, 0 + 2048),
            ) * 2.0
                - 1.0,
            vec3(
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0 + 2048, 0 + 1024),
                hash_noise(x, 0 + 3072, 0 + 2048),
            ) * 2.0
                - 1.0,
        );
    }
}

fn bevy_vector_shapes_immidate(mut shapes: ShapePainter) {
    shapes.thickness = 0.002;
    shapes.cap = Cap::None;
    shapes.disable_laa = true;

    for x in 0..COUNT {
        shapes.line(
            vec3(
                hash_noise(x, 0, 0),
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0, 0 + 2048),
            ) * 2.0
                - 1.0,
            vec3(
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0 + 2048, 0 + 1024),
                hash_noise(x, 0 + 3072, 0 + 2048),
            ) * 2.0
                - 1.0,
        );
    }
}

fn gizmos_immidate(mut gizmos: Gizmos) {
    for x in 0..COUNT {
        gizmos.line(
            vec3(
                hash_noise(x, 0, 0),
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0, 0 + 2048),
            ) * 2.0
                - 1.0,
            vec3(
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0 + 2048, 0 + 1024),
                hash_noise(x, 0 + 3072, 0 + 2048),
            ) * 2.0
                - 1.0,
            Color::WHITE,
        );
    }
}

fn bevy_polyline_retained(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let mut vertices = Vec::with_capacity(COUNT as usize * 2);
    for x in 0..COUNT {
        vertices.push(
            vec3(
                hash_noise(x, 0, 0),
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0, 0 + 2048),
            ) * 2.0
                - 1.0,
        );
        vertices.push(
            vec3(
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0 + 2048, 0 + 1024),
                hash_noise(x, 0 + 3072, 0 + 2048),
            ) * 2.0
                - 1.0,
        );
    }
    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline { vertices }),
        material: polyline_materials.add(PolylineMaterial {
            width: 1.0,
            color: LinearRgba::WHITE,
            perspective: false,
            ..default()
        }),
        ..default()
    });
}

fn bevy_lines_example_retained(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    let mut lines = Vec::with_capacity(COUNT as usize * 2);
    for x in 0..COUNT {
        lines.push((
            vec3(
                hash_noise(x, 0, 0),
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0, 0 + 2048),
            ) * 2.0
                - 1.0,
            vec3(
                hash_noise(x, 0, 0 + 1024),
                hash_noise(x, 0 + 2048, 0 + 1024),
                hash_noise(x, 0 + 3072, 0 + 2048),
            ) * 2.0
                - 1.0,
        ));
    }
    // Spawn a list of lines with start and end points for each lines
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(LineList { lines }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(LineMaterial {
            color: LinearRgba::WHITE,
        }),
        ..default()
    });
}

fn bevy_plane_3d_retained(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::default().mesh().size(0.002, 1.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    for x in 0..COUNT {
        let a = vec3(
            hash_noise(x, 0, 0),
            hash_noise(x, 0, 0 + 1024),
            hash_noise(x, 0, 0 + 2048),
        ) * 2.0
            - 1.0;

        let b = vec3(
            hash_noise(x, 0, 0 + 1024),
            hash_noise(x, 0 + 2048, 0 + 1024),
            hash_noise(x, 0 + 3072, 0 + 2048),
        ) * 2.0
            - 1.0;

        let n = (b - a).normalize();
        let len = (b - a).length();
        let mut transform =
            Transform::from_translation(a + n * len * 0.5).with_scale(vec3(1.0, 1.0, len));
        transform = transform.looking_at(b, vec3(0., 0.0, 3.5));

        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        });
    }
}

// From https://github.com/DGriffin91/bevy_bistro_scene/blob/72c15b37199d994648a3fe43ad569d87c71504d9/src/main.rs#L402
fn benchmark(
    input: Res<ButtonInput<KeyCode>>,
    mut bench_started: Local<Option<Instant>>,
    mut bench_frame: Local<u32>,
    mut count_per_step: Local<u32>,
    time: Res<Time>,
) {
    if input.just_pressed(KeyCode::KeyB) && bench_started.is_none() {
        *bench_started = Some(Instant::now());
        *bench_frame = 0;
        // Try to render for around 5s or at least 30 frames per step
        *count_per_step = ((5.0 / time.delta_seconds()) as u32).max(30);
        println!(
            "Starting Benchmark with {} frames per step",
            *count_per_step
        );
    }
    if bench_started.is_none() {
        return;
    }
    if *bench_frame == *count_per_step {
        let elapsed = bench_started.unwrap().elapsed().as_secs_f32();
        println!(
            "Benchmark avg cpu frame time: {:.2}ms",
            (elapsed / *bench_frame as f32) * 1000.0
        );
        *bench_started = None;
        *bench_frame = 0;
    }
    *bench_frame += 1;
}