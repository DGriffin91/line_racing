//! Create a custom material to draw basic lines in 3D
// From https://github.com/bevyengine/bevy/blob/0c7df881e72c1167060c170b09e02586fc74d85f/examples/3d/lines.rs#L1

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::PrimitiveTopology,
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
            // Add the vertices positions as an attribute
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}
