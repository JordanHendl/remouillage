use crate::utils::texture::*;
use dashi::utils::Handle;
use dashi::*;

use crate::database::*;

pub struct MeshMaterialInfo {
    pub diffuse: Option<Texture>,
    pub specular: Option<Texture>,
}

impl MeshMaterialInfo {
    pub fn from(material: &geometry::Material) -> Self {
        todo!()
    }

    pub fn into_bind_group(&mut self, ctx: &mut dashi::Context) -> BindGroup {
        todo!()
    }
}
pub struct Material {
    pub name: String,
    pub info: MeshMaterialInfo,
    pub bind_group: BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertices: Handle<Buffer>,
    pub indices: Handle<Buffer>,
    pub material: Material,
}
