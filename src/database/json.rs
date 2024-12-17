use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ImageJSONEntry {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ImageJSON {
    pub images: Vec<ImageJSONEntry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AtlasJSONSprite {
    pub name: String,
    pub id: u32,
    pub bounds: dashi::Rect2D,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AtlasJSONAutoGen {
    pub name: String,
    pub bounds: dashi::Rect2D,
    pub stride: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AtlasJSONEntry {
    pub name: String,
    pub path: String,
    pub entries: Option<Vec<AtlasJSONSprite>>,
    pub auto_gen: Option<AtlasJSONAutoGen>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AtlasJSON {
    pub atlases: Vec<AtlasJSONEntry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TTFJSONEntry {
    pub name: String,
    pub path: String,
    pub size: f64,
    pub glyphs: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TTFJSON {
    pub fonts: Vec<TTFJSONEntry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MeshInfo {
    pub name: Option<String>,
    pub material: Option<String>,
    pub render_mask: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GeometryJSONEntry {
    pub name: String,
    pub path: String,
    pub render_mask: String,
    pub meshes: Option<Vec<MeshInfo>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GeometryJSON {
    pub models: Vec<GeometryJSONEntry>, }

#[derive(Deserialize, Serialize, Clone)]
pub struct MaterialJSON {
    pub materials: Vec<super::geometry::Material>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DatabaseJSON {
    pub image_cfg: Option<String>,
    pub atlas_cfg: Option<String>,
    pub geometry_cfg: Option<String>,
    pub ttf_cfg: Option<String>,
    pub particle_cfg: Option<String>,
    pub render_graph_path: Option<String>,
    pub shader_path: Option<String>,
}
