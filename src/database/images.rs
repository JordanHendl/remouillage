use super::json::*;
use super::load_funcs::*;
use super::TTFont;
use std::collections::HashMap;

pub struct ImageEntry {
    pub cfg: ImageJSONEntry,
    pub loaded: Option<ImageLoadInfo<u8>>,
}

impl ImageEntry {
    pub fn load(&mut self, base_path: &str) {
        self.loaded =
            Some(load_image_rgba8(&format!("{}/{}", base_path, self.cfg.path.as_str())).unwrap());
    }

    pub fn unload(&mut self) {
        self.loaded = None;
    }
}

pub struct AtlasEntry {
    pub cfg: AtlasJSONEntry,
    pub loaded: Option<ImageLoadInfo<u8>>,
}

impl AtlasEntry {
    pub fn load(&mut self, base_path: &str) {
        self.loaded =
            Some(load_image_rgba8(&format!("{}/{}", base_path, self.cfg.path.as_str())).unwrap());
    }

    pub fn unload(&mut self) {
        self.loaded = None;
    }
}

pub struct GeometryEntry {
    pub cfg: GeometryJSONEntry,
    pub loaded: Option<super::Model>,
}

impl GeometryEntry {
    pub fn load(&mut self, base_path: &str) {
        if self.loaded.is_none() {
            self.loaded =
                super::load_gltf_model(&format!("{}/{}", base_path, self.cfg.path.as_str()));
        }
    }

    pub fn unload(&mut self) {
        self.loaded = None;
    }
}
pub struct TTFEntry {
    pub cfg: TTFJSONEntry,
    pub loaded: Option<TTFont>,
}

impl TTFEntry {
    pub fn load(&mut self, base_path: &str, typeset: &[char]) {
        self.loaded = Some(TTFont::new(
            &format!("{}/{}", base_path, self.cfg.path.as_str()),
            1280,
            1024,
            self.cfg.size as f32,
            typeset,
        ));
    }

    pub fn unload(&mut self) {
        self.loaded = None;
    }
}
pub fn parse_atlasses(info: AtlasJSON) -> HashMap<String, AtlasEntry> {
    let tup_vec: Vec<(String, AtlasEntry)> = info
        .atlases
        .into_iter()
        .map(|a| {
            (
                a.name.clone(),
                AtlasEntry {
                    cfg: a.clone(),
                    loaded: None,
                },
            )
        })
        .collect();

    return tup_vec.into_iter().collect();
}

pub fn parse_images(info: ImageJSON) -> HashMap<String, ImageEntry> {
    let tup_vec: Vec<(String, ImageEntry)> = info
        .images
        .into_iter()
        .map(|a| {
            (
                a.name.clone(),
                ImageEntry {
                    cfg: a.clone(),
                    loaded: None,
                },
            )
        })
        .collect();

    return tup_vec.into_iter().collect();
}

pub fn parse_ttfs(info: TTFJSON) -> HashMap<String, TTFEntry> {
    let tup_vec: Vec<(String, TTFEntry)> = info
        .fonts
        .into_iter()
        .map(|a| {
            (
                a.name.clone(),
                TTFEntry {
                    cfg: a.clone(),
                    loaded: None,
                },
            )
        })
        .collect();

    return tup_vec.into_iter().collect();
}
