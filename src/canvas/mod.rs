use dashi::utils::*;
use dashi::*;
use serde::{Deserialize, Serialize};
use std::fs;
extern crate unzip3;
use self::unzip3::Unzip3;

#[derive(Serialize, Deserialize)]
struct CanvasAttachment {
    name: String,
    format: Format,
    samples: SampleCount,
    load_op: LoadOp,
    store_op: StoreOp,
    stencil_load_op: LoadOp,
    stencil_store_op: StoreOp,
    clear_color: [f32; 4],
}

struct Attachment {
    img: Handle<Image>,
    view: Handle<ImageView>,
    info: CanvasAttachment,
}

#[derive(Serialize, Deserialize)]
struct CanvasJSONInfo {
    name: String,
    viewport: Viewport,
    color_attachments: Vec<CanvasAttachment>,
    depth_stencil: Option<CanvasAttachment>,
}

pub enum StaticCanvasProfile {
    SIMPLE, // One color & depth attachment.
}

#[allow(dead_code)]
pub struct Canvas {
    name: String,
    viewport: Viewport,
    color_images: Vec<Handle<Image>>,
    color_views: Vec<Handle<ImageView>>,
    depth: Option<Handle<Image>>,
    render_pass: Handle<RenderPass>,
}

impl Canvas {
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }
    pub fn render_pass(&self) -> Handle<RenderPass> {
        return self.render_pass;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn color_attachment(&self, idx: u32) -> Handle<ImageView> {
        self.color_views[idx as usize]
    }

    pub fn from_json(ctx: &mut Context, path: &str) -> Self {
        let json_data = fs::read_to_string(path).expect("Failed to read JSON for Canvas!");
        let info: CanvasJSONInfo =
            serde_json::from_str(&json_data).expect("Failed to read Canvas from JSON!");

        // Fn to convert CanvasAttachment -> tuple (img, view, dashi attachment)
        let mut attach_to_tuple = |a: CanvasAttachment| {
            let img = ctx
                .make_image(&ImageInfo {
                    debug_name: &a.name,
                    dim: [info.viewport.area.w as u32, info.viewport.area.h as u32, 1],
                    format: a.format,
                    mip_levels: 1,
                    initial_data: None,
                })
                .unwrap();

            let view = ctx
                .make_image_view(&ImageViewInfo {
                    debug_name: &a.name,
                    img,
                    layer: 0,
                    mip_level: 0,
                })
                .unwrap();

            let attachment = Attachment {
                view,
                samples: a.samples,
                load_op: a.load_op,
                store_op: a.store_op,
                stencil_load_op: a.stencil_load_op,
                stencil_store_op: a.stencil_store_op,
                clear_color: a.clear_color,
            };

            return (img, view, attachment);
        };

        let colors: Vec<(Handle<Image>, Handle<ImageView>, Attachment)> = info
            .color_attachments
            .into_iter()
            .map(|a| attach_to_tuple(a))
            .collect();

        let (imgs, views, attachs): (Vec<_>, Vec<_>, Vec<_>) = colors.iter().cloned().unzip3();

        let (depth, _view, depth_attach) = match info.depth_stencil {
            Some(a) => {
                let (img, view, attachment) = attach_to_tuple(a);
                (Some(img), Some(view), Some(attachment))
            }
            None => (None, None, None),
        };

        let render_pass = ctx
            .make_render_pass(&RenderPassInfo {
                debug_name: "Shoyu Canvas Render Pass",
                viewport: info.viewport,
                color_attachments: &attachs,
                depth_stencil_attachment: depth_attach.as_ref(),
            })
            .unwrap();

        Self {
            viewport: info.viewport,
            color_images: imgs,
            depth,
            render_pass,
            name: info.name,
            color_views: views,
        }
    }
}
