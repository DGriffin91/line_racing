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
    core::FrameCount,
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
pub struct BenchmarkAllMode;

#[derive(Resource)]
pub struct BenchmarkName(String);

#[derive(Event)]
pub struct UpdateCountEvent(pub u32);

#[derive(Resource)]
pub struct LineCount(pub u32);

#[derive(Component)]
pub struct RetainedLines;

#[derive(Resource)]
pub struct AutoCount;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let auto_bench = args.contains(&"--auto_bench".to_string());
    let verbose = args.contains(&"--verbose".to_string());
    let auto_count = args.contains(&"--auto_count".to_string());

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
            cmd.arg(b).arg("--auto_bench");
            if verbose {
                cmd.arg("--verbose");
            }
            if auto_count {
                cmd.arg("--auto_count");
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
        auto_bench && !verbose,
    );
    app.insert_resource(BenchmarkName(bench_name))
        .insert_resource(CountStable(false));

    if auto_count {
        app.insert_resource(AutoCount)
            .insert_resource(LineCount(50_000));
    } else {
        app.insert_resource(LineCount(150_000));
    }

    if auto_bench {
        app.insert_resource(BenchmarkAllMode);
    } else {
        app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));
    }

    if args.contains(&"--bevy_lines_example_retained".to_string()) {
        app.add_systems(Update, bevy_lines_example_retained);
    }
    if args.contains(&"--bevy_plane_3d_retained".to_string()) {
        app.add_systems(Update, bevy_plane_3d_retained);
    }
    if args.contains(&"--bevy_plane_3d_retained_combined".to_string()) {
        app.add_systems(Update, bevy_plane_3d_retained_combined);
    }
    if args.contains(&"--bevy_polyline_retained".to_string()) {
        app.add_systems(Update, bevy_polyline_retained);
    }
    if args.contains(&"--bevy_polyline_retained_nan".to_string()) {
        app.add_systems(Update, bevy_polyline_retained_nan);
    }
    if args.contains(&"--bevy_polyline_retained_continuous_polyline".to_string()) {
        app.add_systems(Update, bevy_polyline_retained_continuous_polyline);
    }
    if args.contains(&"--bevy_vector_shapes_retained".to_string()) {
        app.add_systems(Update, bevy_vector_shapes_retained);
        //if auto_count {
        //    app.insert_resource(LineCount(10_000));
        //}
    }
    if args.contains(&"--bevy_vector_shapes_immediate".to_string()) {
        app.add_systems(Update, bevy_vector_shapes_immediate);
        //if auto_count {
        //    app.insert_resource(LineCount(10_000));
        //}
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
        .add_systems(Update, (benchmark, line_count_tuner))
        .add_event::<UpdateCountEvent>();
    app
}

fn camera(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    mut update_count_event: EventWriter<UpdateCountEvent>,
    line_count: Res<LineCount>,
) {
    for (_, config, _) in config_store.iter_mut() {
        config.line_width = 1.0;
    }
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        ..default()
    });
    update_count_event.send(UpdateCountEvent(line_count.0));
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
    line_count: Res<LineCount>,
    auto_count: Option<Res<AutoCount>>,
    count_stable: Res<CountStable>,
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

    if (*warm_up_frames > 20 || input.just_pressed(KeyCode::KeyB))
        && bench_started.is_none()
        && (count_stable.0 || auto_count.is_none())
    {
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

        println!(
            "{:>6.1}k lines/ms: {} ({:.1}ms)",
            (line_count.0 as f32 / time_ms) / 1000.0,
            benchmark_name.0,
            time_ms,
        );

        *bench_started = None;
        *bench_frame = 0;
        if all_benchmark_mode.is_some() {
            app_exit.send(bevy::app::AppExit::Success);
        }
    }
    *bench_frame += 1;
}

#[derive(Resource)]
pub struct CountStable(pub bool);

fn line_count_tuner(
    mut commands: Commands,
    mut update_count_event: EventWriter<UpdateCountEvent>,
    benchmark_name: Res<BenchmarkName>,
    mut line_count: ResMut<LineCount>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    retained_lines: Query<Entity, With<RetainedLines>>,
    mut count_stable: ResMut<CountStable>,
    frame_count: Res<FrameCount>,
    mut updated_last_frame: Local<u32>,
    mut sit_time: Local<u32>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stability: Local<u32>,
    mut last_time: Local<f32>,
    auto_count: Option<Res<AutoCount>>,
) {
    if count_stable.0 == true || auto_count.is_none() {
        return;
    }
    if *sit_time == 0 {
        *sit_time = 20;
    }
    if *updated_last_frame + *sit_time > frame_count.0 {
        return;
    }
    let avg_time = (*last_time + time.delta_seconds()) * 0.5;
    *last_time = time.delta_seconds();
    let this_time_ms = avg_time * 1000.0;
    let mut updated = false;

    if this_time_ms < 8.0 {
        line_count.0 *= 2;
        update_count_event.send(UpdateCountEvent(line_count.0));
        updated = true;
        *stability = 0;
        *updated_last_frame = frame_count.0;
    } else {
        *stability += 1;
    }

    if *stability == 100 {
        count_stable.0 = true;
        return;
    }

    if updated {
        let mut window = windows.single_mut();
        window.title = format!("{}: {} lines", benchmark_name.0, line_count.0);
        for entity in retained_lines.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let mesh_hs = meshes.iter().map(|(h, _)| h).collect::<Vec<_>>();
        for mesh in &mesh_hs {
            meshes.remove(*mesh);
        }
    }
}
