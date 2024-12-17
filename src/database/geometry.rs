use glam::*;
pub type Index = u32;
use glam::{Vec2, Vec3};
use gltf::Gltf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub bone_ids: [u32; 4],
    pub bone_weights: [f32; 4],
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub enum TextureType {
    Diffuse,
    Specular,
    Roughness,
    Normal,
    Occlusion,
    Emissive,
    Albedo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub textures: HashMap<TextureType, String>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub fn load_gltf_model<P: AsRef<Path>>(path: P) -> Option<Model> {
    let (gltf, buffers, _) = gltf::import(path).expect("Failed to load glTF file");
    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut material = None;
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut bone_ids: Vec<[u16; 4]> = Vec::new();
            let mut bone_weights: Vec<[f32; 4]> = Vec::new();

            // Extract Positions
            let positions: Vec<[f32; 3]> = reader.read_positions()?.collect();

            // Extract Normals
            let normals: Vec<[f32; 3]> = reader.read_normals()?.collect();

            // Extract Texture Coordinates
            let tex_coords: Vec<[f32; 2]> = reader.read_tex_coords(0)?.into_f32().collect();

            // Extract Bone Weights
            if let Some(joints) = reader.read_joints(0) {
                bone_ids = joints.into_u16().collect();
            } else {
                bone_ids.resize(positions.len(), [0, 0, 0, 0]);
            }

            if let Some(weights) = reader.read_weights(0) {
                bone_weights = weights.into_f32().collect();
            } else {
                bone_weights.resize(positions.len(), [0.0, 0.0, 0.0, 0.0]);
            }

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
            
            let mut mat_name = "Unknown".to_string();
            // Extract Material Information
            let mat = primitive.material();
            {
                if let Some(name) = mat.name() {
                    mat_name = name.to_string();
                }
                let mut textures = HashMap::new();

                if let Some(info) = mat.pbr_metallic_roughness().base_color_texture() {
                    if let Some(name) = info.texture().name() {
                        textures.insert(TextureType::Diffuse, name.to_string());
                    }
                }

                if let Some(info) = mat.normal_texture() {
                    if let Some(name) = info.texture().name() {
                        textures.insert(TextureType::Normal, name.to_string());
                    }
                }

                if let Some(info) = mat.occlusion_texture() {
                    if let Some(name) = info.texture().name() {
                        textures.insert(TextureType::Occlusion, name.to_string());
                    }
                }

                if let Some(info) = mat.emissive_texture() {
                    if let Some(name) = info.texture().name() {
                        textures.insert(TextureType::Emissive, name.to_string());
                    }
                }

                material = Some(Material { name: mat_name, textures });
            }
        }

        meshes.push(Mesh {
            vertices,
            indices,
            material: material.unwrap(),
        });
    }

    Some(Model { meshes })
}
