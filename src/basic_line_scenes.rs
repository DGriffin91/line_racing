use core::f32;

use bevy::{math::vec3, prelude::*};
use bevy_mod_mesh_tools::{mesh_append, mesh_empty_default, mesh_with_transform};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_vector_shapes::prelude::*;

use crate::{
    bevy_lines_example::{LineList, LineMaterial},
    sampling::ContinuousRandomLineGenerator,
    LineCount,
};

pub fn bevy_vector_shapes_retained(mut shapes: ShapeCommands, count: Res<LineCount>) {
    shapes.thickness = 0.002;
    shapes.cap = Cap::None;
    shapes.disable_laa = true;

    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
        shapes.line(line.0, line.1);
    }
}

pub fn bevy_vector_shapes_immediate(mut shapes: ShapePainter, count: Res<LineCount>) {
    shapes.thickness = 0.002;
    shapes.cap = Cap::None;
    shapes.disable_laa = true;

    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
        shapes.line(line.0, line.1);
    }
}

pub fn gizmos_immediate(mut gizmos: Gizmos, count: Res<LineCount>) {
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
        gizmos.line(line.0, line.1, Color::WHITE);
    }
}

pub fn gizmos_immediate_nan(mut gizmos: Gizmos, count: Res<LineCount>) {
    // Draws a single polyline (instead of individual lines) and inserts a NaN in between lines to separate them.
    let mut vertices = Vec::with_capacity(count.0 as usize * 3);
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
        vertices.push(line.0);
        vertices.push(line.1);
        vertices.push(Vec3::splat(f32::NAN));
    }
    gizmos.linestrip(vertices.clone(), Color::WHITE)
}

pub fn gizmos_immediate_continuous_polyline(mut gizmos: Gizmos, count: Res<LineCount>) {
    // Draws a single polyline (instead of individual lines).
    let mut vertices = Vec::with_capacity(count.0 as usize);
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next();
        vertices.push(line);
    }
    gizmos.linestrip(vertices.clone(), Color::WHITE)
}

pub fn bevy_polyline_retained_continuous_polyline(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    count: Res<LineCount>,
) {
    // Draws a single polyline (instead of individual lines).
    let mut vertices = Vec::with_capacity(count.0 as usize);
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
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

pub fn bevy_polyline_retained_nan(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    count: Res<LineCount>,
) {
    // Draws a single polyline (instead of individual lines) and inserts a NaN in between lines to separate them.
    let mut vertices = Vec::with_capacity(count.0 as usize * 3);
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
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

pub fn bevy_polyline_retained(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    count: Res<LineCount>,
) {
    let material = polyline_materials.add(PolylineMaterial {
        width: 1.0,
        color: LinearRgba::WHITE,
        perspective: false,
        ..default()
    });
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
        commands.spawn(PolylineBundle {
            polyline: polylines.add(Polyline {
                vertices: vec![line.0, line.1],
            }),
            material: material.clone(),
            ..default()
        });
    }
}

pub fn bevy_lines_example_retained(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    count: Res<LineCount>,
) {
    let mut lines = Vec::with_capacity(count.0 as usize * 2);
    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
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

pub fn bevy_plane_3d_retained(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    count: Res<LineCount>,
) {
    // Need to oversize just a tad so the planes can rasterize a bit more greedily like the actual line primitives
    let mesh = meshes.add(Plane3d::default().mesh().size(0.0022, 1.01));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
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

pub fn bevy_plane_3d_retained_combined(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    count: Res<LineCount>,
) {
    // Combines all the individual line meshes into one single mesh.

    // Need to oversize just a tad so the planes can rasterize a bit more greedily like the actual line primitives
    let mesh = Plane3d::default().mesh().size(0.0022, 1.01).build();
    let mut combined_mesh = mesh_empty_default();

    let mut line_gen = ContinuousRandomLineGenerator::default();
    for _ in 0..count.0 {
        let line = line_gen.next_line();
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
