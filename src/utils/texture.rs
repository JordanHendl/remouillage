pub struct Texture {
    pub name: String,
    pub handle: dashi::utils::Handle<dashi::Image>,
    pub dim: [u32; 3],
}
