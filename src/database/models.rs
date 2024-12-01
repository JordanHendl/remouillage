use glam::{Vec2, Vec3};
use gltf::Gltf;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub bone_ids: [u32; 4],
    pub bone_weights: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub fn parse_gltf_model<P: AsRef<Path>>(path: P) -> Model {
    let (gltf, buffers, _) = gltf::import(path).expect("Failed to load glTF file");
    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // Extract Positions
            let positions: Vec<[f32; 3]> = reader
                .read_positions()
                .expect("Missing vertex positions")
                .collect();

            // Extract Normals
            let normals: Vec<[f32; 3]> = reader
                .read_normals()
                .expect("Missing vertex normals")
                .collect();

            // Extract Texture Coordinates
            let tex_coords: Vec<[f32; 2]> = reader
                .read_tex_coords(0)
                .expect("Missing texture coordinates")
                .into_f32()
                .collect();

            // Extract Bone Weights
            let bone_ids: Vec<[u16; 4]> = reader
                .read_joints(0)
                .expect("Missing bone joint ids")
                .into_u16()
                .collect();

            let bone_weights: Vec<[f32; 4]> = reader
                .read_weights(0)
                .expect("Missing bone weights")
                .into_f32()
                .collect();

            // Collect vertex data
            for i in 0..positions.len() {
                vertices.push(Vertex {
                    pos: Vec3::from(positions[i]),
                    normal: Vec3::from(normals[i]),
                    tex_coords: Vec2::from(tex_coords[i]),
                    bone_ids: [
                        bone_ids[i][0] as u32,
                        bone_ids[i][1] as u32,
                        bone_ids[i][2] as u32,
                        bone_ids[i][3] as u32,
                    ],
                    bone_weights: bone_weights[i],
                });
            }

            // Extract Indices
            if let Some(indices_data) = reader.read_indices() {
                indices.extend(indices_data.into_u32());
            }
        }

        meshes.push(Mesh { vertices, indices });
    }

    Model { meshes }
}
