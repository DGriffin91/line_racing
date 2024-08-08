// Demonstrates using ShapeCommands to spawn entity backed shapes

pub mod basic_line_scenes;
pub mod bevy_lines_example;
pub mod sampling;

use core::f32;
use std::{f32::consts::TAU, process::Command, time::Instant};

use basic_line_scenes::{
    bevy_lines_example_retained, bevy_plane_3d_retained, bevy_plane_3d_retained_combined,
    bevy_polyline_retained, bevy_polyline_retained_continuous_polyline, bevy_polyline_retained_nan,
    bevy_vector_shapes_immediate, bevy_vector_shapes_retained, gizmos_immediate,
    gizmos_immediate_continuous_polyline, gizmos_immediate_nan,
};
use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
    winit::{UpdateMode, WinitSettings},
};
use bevy_lines_example::LineMaterial;
use bevy_polyline::PolylinePlugin;
use bevy_vector_shapes::prelude::*;

#[derive(Resource)]
struct BenchmarkAllMode;

#[derive(Resource)]
struct BenchmarkName(String);

#[derive(Resource)]
pub struct LineCount(u32);

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
        .insert_resource(LineCount(COUNT));
    app
}

const COUNT: u32 = 150_000;

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

// From https://github.com/DGriffin91/bevy_bistro_scene/blob/72c15b37199d994648a3fe43ad569d87c71504d9/src/main.rs#L402
fn benchmark(
    input: Res<ButtonInput<KeyCode>>,
    mut bench_started: Local<Option<Instant>>,
    mut bench_frame: Local<u32>,
    mut count: Local<u32>,
    benchmark_name: Res<BenchmarkName>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    all_benchmark_mode: Option<Res<BenchmarkAllMode>>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
    mut start_time: Local<f32>,
    mut warm_up_frames: Local<u32>,
) {
    if all_benchmark_mode.is_some() {
        if *start_time == 0.0 {
            *start_time = time.elapsed_seconds();
        }
        // Warm up for 2 seconds from the time this function in the Update schedule was first able to run, and then for an additional 20 frames
        if time.elapsed_seconds() - *start_time > 2.0 {
            *warm_up_frames += 1;
        }
    }

    if (*warm_up_frames > 20 || input.just_pressed(KeyCode::KeyB)) && bench_started.is_none() {
        *bench_started = Some(Instant::now());
        *bench_frame = 0;
        // Try to render for around 4s or at least 30 frames
        *count = ((4.0 / time.delta_seconds()) as u32).max(30);
        if all_benchmark_mode.is_none() {
            println!("Starting Benchmark with {} frames", *count);
        }
    }
    if bench_started.is_none() {
        return;
    }

    let mut camera = camera.single_mut();
    let t = (*bench_frame as f32 / *count as f32) * TAU;
    camera.translation.x = t.sin() * 3.5;
    camera.translation.z = t.cos() * 3.5;
    *camera = camera.looking_at(Vec3::ZERO, Vec3::Y);

    if *bench_frame == *count {
        let elapsed = bench_started.unwrap().elapsed().as_secs_f32();
        let time_ms = (elapsed / *bench_frame as f32) * 1000.0;
        println!("{:>6.2}ms: {}", time_ms, benchmark_name.0);
        *bench_started = None;
        *bench_frame = 0;
        if all_benchmark_mode.is_some() {
            app_exit.send(bevy::app::AppExit::Success);
        }
    }
    *bench_frame += 1;
}
