use bevy::prelude::*;
use bevy::render::{
    mesh::Indices,
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};

/// Flat pointy-top hex ring on the XZ plane (Y-up), aligned with map tiles.
pub fn hex_outline_mesh(outer_radius: f32, inner_radius: f32) -> Mesh {
    let mut positions = Vec::with_capacity(24);
    let mut normals = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    for i in 0..6 {
        let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::FRAC_PI_3;
        let next_angle = angle + std::f32::consts::FRAC_PI_3;

        let (cos_a, sin_a) = angle.sin_cos();
        let (cos_b, sin_b) = next_angle.sin_cos();

        let base = positions.len() as u32;
        positions.extend([
            [outer_radius * cos_a, 0.0, outer_radius * sin_a],
            [outer_radius * cos_b, 0.0, outer_radius * sin_b],
            [inner_radius * cos_b, 0.0, inner_radius * sin_b],
            [inner_radius * cos_a, 0.0, inner_radius * sin_a],
        ]);
        normals.extend([[0.0, 1.0, 0.0]; 4]);
        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
