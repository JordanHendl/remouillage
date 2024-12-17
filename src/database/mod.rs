pub mod error;
pub mod json;
pub mod geometry;
pub mod load_funcs;
mod images;
use std::collections::HashMap;
use std::fs;
use json::*;
use error::*;
use geometry::*;
use images::*;
pub mod font;
pub use font::*;

pub struct Database {
    base_path: String,
    images: HashMap<String, ImageEntry>,
    atlases: HashMap<String, AtlasEntry>,
//    geometry: HashMap<String, GeometryEntry>,
    ttfs: HashMap<String, TTFEntry>,
    particle_cfg: String,
}

impl Database {
    fn get_images_json(path: &str) -> Result<ImageJSON, Error> {
        let json_data = fs::read_to_string(path)?;
        let info: ImageJSON = serde_json::from_str(&json_data)?;
        Ok(info)
    }

    fn get_atlases_json(path: &str) -> Result<AtlasJSON, Error> {
        let json_data = fs::read_to_string(path)?;
        let info: AtlasJSON = serde_json::from_str(&json_data)?;
        Ok(info)
    }
    
    fn get_ttf_json(path: &str) -> Result<TTFJSON, Error> {
        let json_data = fs::read_to_string(path)?;
        let info: TTFJSON = serde_json::from_str(&json_data)?;
        Ok(info)
    }
    
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    pub fn new(base_path: &str) -> Result<Self, Error> {
        let json_data = fs::read_to_string(format!("{}/database.json", base_path))?;

        let info: DatabaseJSON = serde_json::from_str(&json_data)?;

        let images = if let Some(sprite) = info.image_cfg {
            parse_images(Database::get_images_json(&format!(
                "{}/{}",
                base_path,
                sprite.as_str()
            ))?)
        } else {
            HashMap::new()
        };

        let atlases = if let Some(sprite) = info.atlas_cfg {
            parse_atlasses(Database::get_atlases_json(&format!(
                "{}/{}",
                base_path,
                sprite.as_str()
            ))?)
        } else {
            HashMap::new()
        };

        let ttfs = if let Some(ttf) = info.ttf_cfg {
            parse_ttfs(Database::get_ttf_json(&format!(
                "{}/{}",
                base_path,
                ttf.as_str()
            ))?)
        } else {
            HashMap::new()
        };

        Ok(Database {
            base_path: base_path.to_string(),
            images,
            atlases,
            ttfs,
            particle_cfg: if info.particle_cfg.is_some() {
                info.particle_cfg.unwrap().clone()
            } else {
                "".to_string()
            },
        })
    }

    pub fn particle_system_cfg_path(&self) -> Result<String, Error> {
        return Ok(self.particle_cfg.clone());
    }

//    pub fn fetch_sprite(&mut self, name: &str) -> Result<&ImageEntry, Error> {
//        // TODO probably async this.
//        if let Some(entry) = self.sprites.get_mut(name) {
//            if entry.loaded.is_none() {
//                entry.load(&self.base_path);
//            }
//
//            return Ok(entry);
//        }
//
//        return Err(Error::LookupError(LookupError {
//            entry: name.to_string(),
//        }));
//    }

    pub fn fetch_ttf(&mut self, name: &str) -> Result<&TTFEntry, Error> {
        let default_typeset: Vec<char> = (0 as u8 as char..127 as u8 as char).collect();
        // TODO probably async this.
        if let Some(entry) = self.ttfs.get_mut(name) {
            if entry.loaded.is_none() {
                #[allow(unused_assignments)]
                let mut str = Vec::new();
                let glyphs: &[char] = match entry.cfg.glyphs.clone() {
                    Some(g) => {
                        str = g.chars().collect();
                        &str
                    }
                    None => &default_typeset,
                };
                entry.load(&self.base_path, glyphs);
            }

            return Ok(entry);
        }

        return Err(Error::LookupError(LookupError {
            entry: name.to_string(),
        }));
    }

//    pub fn fetch_sprite_sheet(&mut self, name: &str) -> Result<&AtlasEntry, Error> {
//        // TODO probably async this.
//        if let Some(entry) = self.sprite_sheets.get_mut(name) {
//            if entry.loaded.is_none() {
//                entry.load(&self.base_path);
//            }
//
//            return Ok(entry);
//        }
//
//        return Err(Error::LookupError(LookupError {
//            entry: name.to_string(),
//        }));
//    }
}

#[test]
fn test_database() {
    let res = Database::new("/wksp/database");
    assert!(res.is_ok());

    let mut db = res.unwrap();
//    let sprite = db.fetch_sprite("name");
//    assert!(sprite.is_ok());
//
//    let sprite = db.fetch_sprite_sheet("name");
//    assert!(sprite.is_ok());
}
