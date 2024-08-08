// Demonstrates using ShapeCommands to spawn entity backed shapes

pub mod bevy_lines_example;
pub mod sampling;

use core::f32;
use std::{process::Command, time::Instant};

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    math::vec3,
    prelude::*,
    window::{PresentMode, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
use bevy_lines_example::{LineList, LineMaterial};
use bevy_mod_mesh_tools::{mesh_append, mesh_empty_default, mesh_with_transform};
use bevy_polyline::{
    prelude::{Polyline, PolylineBundle, PolylineMaterial},
    PolylinePlugin,
};
use bevy_vector_shapes::prelude::*;
use sampling::hash_noise;

#[derive(Resource)]
struct BenchmarkAllMode;

#[derive(Resource)]
struct BenchmarkName(String);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let auto_bench_child_process = args.contains(&"--auto_bench_child_process".to_string());
    let verbose = args.contains(&"--verbose".to_string());

    if args.contains(&"--benchmark".to_string()) {
        let program_name = &args[0];
        for b in [
            "--bevy_lines_example_retained",
            "--bevy_plane_3d_retained",
            "--bevy_plane_3d_retained_combined",
            "--gizmos_immediate",
            "--gizmos_immediate_nan",
            "--gizmos_immediate_continuous_polyline",
            "--bevy_vector_shapes_retained",
            "--bevy_vector_shapes_immediate",
            "--bevy_polyline_retained",
            "--bevy_polyline_retained_nan",
            "--bevy_polyline_retained_continuous_polyline",
        ] {
            let mut cmd = Command::new(program_name);
            cmd.arg(b).arg("--auto_bench_child_process");
            if verbose {
                cmd.arg("--verbose");
            }
            let mut child = cmd.spawn().unwrap();
            child.wait().unwrap();
        }
        return;
    }

    // TODO: don't be silly
    let bench_name = args[1].to_string().replace("--", "");
    let mut app = base_app(
        &format!("line racer: {}", bench_name),
        auto_bench_child_process && !verbose,
    );
    app.insert_resource(BenchmarkName(bench_name));

    if auto_bench_child_process {
        app.insert_resource(BenchmarkAllMode);
    } else {
        app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));
    }

    if args.contains(&"--bevy_lines_example_retained".to_string()) {
        app.add_systems(Startup, bevy_lines_example_retained);
    }
    if args.contains(&"--bevy_plane_3d_retained".to_string()) {
        app.add_systems(Startup, bevy_plane_3d_retained);
    }
    if args.contains(&"--bevy_plane_3d_retained_combined".to_string()) {
        app.add_systems(Startup, bevy_plane_3d_retained_combined);
    }
    if args.contains(&"--bevy_polyline_retained".to_string()) {
        app.add_systems(Startup, bevy_polyline_retained);
    }
    if args.contains(&"--bevy_polyline_retained_nan".to_string()) {
        app.add_systems(Startup, bevy_polyline_retained_nan);
    }
    if args.contains(&"--bevy_polyline_retained_continuous_polyline".to_string()) {
        app.add_systems(Startup, bevy_polyline_retained_continuous_polyline);
    }
    if args.contains(&"--bevy_vector_shapes_retained".to_string()) {
        app.add_systems(Startup, bevy_vector_shapes_retained);
    }
    if args.contains(&"--bevy_vector_shapes_immediate".to_string()) {
        app.add_systems(Update, bevy_vector_shapes_immediate);
    }
    if args.contains(&"--gizmos_immediate".to_string()) {
        app.add_systems(Update, gizmos_immediate);
    }
    if args.contains(&"--gizmos_immediate_nan".to_string()) {
        app.add_systems(Update, gizmos_immediate_nan);
    }
    if args.contains(&"--gizmos_immediate_continuous_polyline".to_string()) {
        app.add_systems(Update, gizmos_immediate_continuous_polyline);
    }

    app.run();
}

fn base_app(title: &str, disable_log: bool) -> App {
    let mut default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: title.to_string(),
            present_mode: PresentMode::Immediate,
            resolution: WindowResolution::new(1024.0, 1024.0).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    });
    if disable_log {
        default_plugins = default_plugins.disable::<LogPlugin>();
    }
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .add_plugins(default_plugins)
        .add_plugins((
            ShapePlugin::default(),
            PolylinePlugin,
            MaterialPlugin::<LineMaterial>::default(),
        ))
        .add_systems(Startup, camera)
        .add_systems(Update, benchmark)
        .add_systems(Update, all_benchmark);
    app
}

const TINY_LINES: bool = false;

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
        let line = rng_line(x);
        shapes.line(line.0, line.1);
    }
}

fn bevy_vector_shapes_immediate(mut shapes: ShapePainter) {
    shapes.thickness = 0.002;
    shapes.cap = Cap::None;
    shapes.disable_laa = true;

    for x in 0..COUNT {
        let line = rng_line(x);
        shapes.line(line.0, line.1);
    }
}

fn gizmos_immediate(mut gizmos: Gizmos) {
    for x in 0..COUNT {
        let line = rng_line(x);
        gizmos.line(line.0, line.1, Color::WHITE);
    }
}

fn gizmos_immediate_nan(mut gizmos: Gizmos) {
    // Draws a single polyline (instead of individual lines) and inserts a NaN in between lines to separate them.
    let mut vertices = Vec::with_capacity(COUNT as usize * 3);
    for x in 0..COUNT {
        let line = rng_line(x);
        vertices.push(line.0);
        vertices.push(line.1);
        vertices.push(Vec3::splat(f32::NAN));
    }
    gizmos.linestrip(vertices.clone(), Color::WHITE)
}

fn gizmos_immediate_continuous_polyline(mut gizmos: Gizmos) {
    // Draws a single polyline (instead of individual lines).
    let mut vertices = Vec::with_capacity(COUNT as usize);
    for x in 0..COUNT {
        let line = rng_line(x);
        vertices.push(line.0);
    }
    gizmos.linestrip(vertices.clone(), Color::WHITE)
}

fn bevy_polyline_retained_continuous_polyline(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    // Draws a single polyline (instead of individual lines).
    let mut vertices = Vec::with_capacity(COUNT as usize);
    for x in 0..COUNT {
        let line = rng_line(x);
        vertices.push(line.0);
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

fn bevy_polyline_retained_nan(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    // Draws a single polyline (instead of individual lines) and inserts a NaN in between lines to separate them.
    let mut vertices = Vec::with_capacity(COUNT as usize * 3);
    for x in 0..COUNT {
        let line = rng_line(x);
        vertices.push(line.0);
        vertices.push(line.1);
        vertices.push(Vec3::splat(f32::NAN));
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

fn bevy_polyline_retained(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let material = polyline_materials.add(PolylineMaterial {
        width: 1.0,
        color: LinearRgba::WHITE,
        perspective: false,
        ..default()
    });
    for x in 0..COUNT {
        let line = rng_line(x);
        commands.spawn(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: vec![line.0, line.1],
            }),
            material: material.clone(),
            ..default()
        });
    }
}

fn bevy_lines_example_retained(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    let mut lines = Vec::with_capacity(COUNT as usize * 2);
    for x in 0..COUNT {
        let line = rng_line(x);
        lines.push(line);
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
        let line = rng_line(x);
        let n = (line.1 - line.0).normalize();
        let len = (line.1 - line.0).length();
        let mut transform =
            Transform::from_translation(line.0 + n * len * 0.5).with_scale(vec3(1.0, 1.0, len));
        transform = transform.looking_at(line.1, vec3(0., 0.0, 3.5));

        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        });
    }
}

fn bevy_plane_3d_retained_combined(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Combines all the individual line meshes into one single mesh.
    let mesh = Plane3d::default().mesh().size(0.002, 1.0).build();
    let mut combined_mesh = mesh_empty_default();

    for x in 0..COUNT {
        let line = rng_line(x);
        let n = (line.1 - line.0).normalize();
        let len = (line.1 - line.0).length();
        let mut transform =
            Transform::from_translation(line.0 + n * len * 0.5).with_scale(vec3(1.0, 1.0, len));
        transform = transform.looking_at(line.1, vec3(0., 0.0, 3.5));

        let mesh = mesh_with_transform(&mesh, &transform).unwrap();
        mesh_append(&mut combined_mesh, &mesh).unwrap();
    }

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(combined_mesh),
        material: material.clone(),
        ..default()
    });
}

fn rng_line(x: u32) -> (Vec3, Vec3) {
    let a = vec3(
        hash_noise(x, 1, 0),
        hash_noise(x, 2, 0),
        hash_noise(x, 3, 0),
    ) * 2.0
        - 1.0;
    let b = vec3(
        hash_noise(x, 4, 0),
        hash_noise(x, 5, 0),
        hash_noise(x, 6, 0),
    ) * 2.0
        - 1.0;
    if TINY_LINES {
        (a, a + b * 0.005)
    } else {
        (a, b)
    }
}

// From https://github.com/DGriffin91/bevy_bistro_scene/blob/72c15b37199d994648a3fe43ad569d87c71504d9/src/main.rs#L402
fn benchmark(
    input: Res<ButtonInput<KeyCode>>,
    mut bench_started: Local<Option<Instant>>,
    mut bench_frame: Local<u32>,
    mut count_per_step: Local<u32>,
    benchmark_name: Res<BenchmarkName>,
    time: Res<Time>,
) {
    if input.just_pressed(KeyCode::KeyB) && bench_started.is_none() {
        *bench_started = Some(Instant::now());
        *bench_frame = 0;
        // Try to render for around 4s or at least 30 frames per step
        *count_per_step = ((4.0 / time.delta_seconds()) as u32).max(30);
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
        let time_ms = (elapsed / *bench_frame as f32) * 1000.0;
        println!("{:>6.2}ms: {}", time_ms, benchmark_name.0);
        *bench_started = None;
        *bench_frame = 0;
    }
    *bench_frame += 1;
}

fn all_benchmark(
    mut bench_started: Local<Option<Instant>>,
    mut bench_frame: Local<u32>,
    mut count_per_step: Local<u32>,
    time: Res<Time>,
    all_benchmark_mode: Option<Res<BenchmarkAllMode>>,
    benchmark_name: Res<BenchmarkName>,
    mut warm_up_frames: Local<u32>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
    mut start_time: Local<f32>,
) {
    let Some(_) = all_benchmark_mode else {
        return;
    };
    if *start_time == 0.0 {
        *start_time = time.elapsed_seconds();
    }
    // Warm up for 2 seconds from the time this function in the Update schedule was first able to run, and then for an additional 20 frames
    if time.elapsed_seconds() - *start_time > 2.0 {
        *warm_up_frames += 1;
    }
    if *warm_up_frames > 20 && bench_started.is_none() {
        *bench_started = Some(Instant::now());
        *bench_frame = 0;
        // Try to render for around 4s or at least 30 frames
        *count_per_step = ((4.0 / time.delta_seconds()) as u32).max(30);
    }
    if bench_started.is_none() {
        return;
    }
    if *bench_frame == *count_per_step {
        let elapsed = bench_started.unwrap().elapsed().as_secs_f32();
        let time_ms = (elapsed / *bench_frame as f32) * 1000.0;
        println!("{:>6.2}ms: {}", time_ms, benchmark_name.0);
        *bench_started = None;
        *bench_frame = 0;
        app_exit.send(bevy::app::AppExit::Success);
    }
    *bench_frame += 1;
}
