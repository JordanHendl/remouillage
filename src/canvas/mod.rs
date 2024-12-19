use dashi::utils::*;
use dashi::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
extern crate unzip3;
use self::unzip3::Unzip3;

#[derive(Clone, Serialize, Deserialize)]
pub struct CanvasAttachment {
    pub name: String,
    pub format: Format,
    pub samples: SampleCount,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub clear_color: [f32; 4],
}

#[derive(Clone)]
pub struct CanvasAttachmentDetail {
    pub img: Handle<Image>,
    pub view: Handle<ImageView>,
    pub info: CanvasAttachment,
}

#[derive(Serialize, Deserialize)]
pub struct CanvasCreateInfo {
    pub name: String,
    pub viewport: Viewport,
    pub color_attachments: Vec<CanvasAttachment>,
    pub depth_stencil: Option<CanvasAttachment>,
}

#[allow(dead_code)]
pub struct Canvas {
    name: String,
    viewport: Viewport,
    attach_map: HashMap<String, CanvasAttachmentDetail>,
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

    pub fn color_attachment_by_name(&self, name: &str) -> Option<&CanvasAttachmentDetail> {
        return self.attach_map.get(name);
    }

    pub fn new(ctx: &mut Context, info: CanvasCreateInfo) -> Self {
        let mut map = HashMap::new();

        // Fn to convert CanvasAttachment -> tuple (img, view, dashi attachment)
        let mut attach_to_tuple = |a: CanvasAttachment| {
            let img = ctx
                .make_image(&ImageInfo {
                    debug_name: &a.name,
                    dim: [info.viewport.area.w as u32, info.viewport.area.h as u32, 1],
                    format: a.format,
                    mip_levels: 1,
                    initial_data: None,
                    ..Default::default()
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
            
            map.insert(a.name.clone(), CanvasAttachmentDetail {
                img,
                view,
                info: a.clone(),
            });

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
            attach_map: map,
            name: info.name,
            color_views: views,
        }
    }
    pub fn from_json(ctx: &mut Context, path: &str) -> Self {
        let json_data = fs::read_to_string(path).expect("Failed to read JSON for Canvas!");
        let info: CanvasCreateInfo =
            serde_json::from_str(&json_data).expect("Failed to read Canvas from JSON!");

        return Canvas::new(ctx, info);
    }
}
