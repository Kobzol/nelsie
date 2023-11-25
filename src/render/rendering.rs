use super::text::render_text;
use crate::model::{Color, Drawing, Node, NodeChild, NodeContent, Step};
use crate::render::core::RenderConfig;
use crate::render::layout::{ComputedLayout, LayoutContext};
use std::collections::BTreeSet;

use resvg::tiny_skia;
use std::rc::Rc;

use crate::render::image::render_image;
use crate::render::paths::create_path;
use crate::render::GlobalResources;
use usvg::Fill;

pub(crate) struct RenderContext<'a> {
    global_res: &'a GlobalResources,
    step: Step,
    z_level: i32,
    layout: &'a ComputedLayout,
    svg_node: usvg::Node,
}

impl From<&Color> for usvg::Color {
    fn from(value: &Color) -> Self {
        let c: svgtypes::Color = value.into();
        usvg::Color::new_rgb(c.red, c.green, c.blue)
    }
}

impl<'a> RenderContext<'a> {
    pub fn new(
        global_res: &'a GlobalResources,
        step: Step,
        z_level: i32,
        layout: &'a ComputedLayout,
        svg_node: usvg::Node,
    ) -> Self {
        RenderContext {
            global_res,
            step,
            z_level,
            layout,
            svg_node,
        }
    }

    fn render_helper(&self, node: &Node) {
        if !node.show.at_step(self.step) {
            return;
        }
        if *node.z_level.at_step(self.step) == self.z_level {
            if let Some(color) = &node.bg_color.at_step(self.step) {
                let rect = self.layout.rect(node.node_id).unwrap();
                let mut path = usvg::Path::new(Rc::new(tiny_skia::PathBuilder::from_rect(
                    tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width, rect.height).unwrap(),
                )));
                path.fill = Some(Fill {
                    paint: usvg::Paint::Color(color.into()),
                    ..Default::default()
                });
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(path)));
            }

            if let Some(content) = &node.content.at_step(self.step) {
                let rect = self.layout.rect(node.node_id).unwrap();
                match content {
                    NodeContent::Text(text) => {
                        self.svg_node.append(render_text(&text, rect.x, rect.y));
                    }
                    NodeContent::Image(image) => {
                        render_image(self.global_res, self.step, image, rect, &self.svg_node)
                    }
                }
            }
        }

        for child in &node.children {
            match child {
                NodeChild::Node(node) => self.render_helper(node),
                NodeChild::Draw(draw) => self.draw(draw),
            }
        }
    }

    fn draw(&self, drawing: &Drawing) {
        for path in drawing.paths.at_step(self.step) {
            if let Some(usvg_path) = create_path(self.layout, path) {
                self.svg_node
                    .append(usvg::Node::new(usvg::NodeKind::Path(usvg_path)));
            }
        }
    }

    pub(crate) fn render_to_svg(self, node: &Node) {
        self.render_helper(node);
    }
}

pub(crate) fn render_to_svg_tree(render_cfg: &RenderConfig) -> usvg_tree::Tree {
    log::debug!("Creating layout");
    let layout_builder = LayoutContext::new(render_cfg.global_res, render_cfg.step);
    let layout = layout_builder.compute_layout(render_cfg.slide);

    log::debug!("Layout {:?}", layout);

    let mut z_levels = BTreeSet::new();
    render_cfg.slide.node.collect_z_levels(&mut z_levels);

    log::debug!("Rendering to svg");
    let root_svg_node = usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default()));
    for z_level in z_levels {
        let render_ctx = RenderContext::new(
            render_cfg.global_res,
            render_cfg.step,
            z_level,
            &layout,
            root_svg_node.clone(),
        );
        render_ctx.render_to_svg(&render_cfg.slide.node);
    }
    let size = usvg::Size::from_wh(render_cfg.slide.width, render_cfg.slide.height).unwrap();

    usvg_tree::Tree {
        size,
        view_box: usvg::ViewBox {
            rect: size.to_non_zero_rect(0.0, 0.0),
            aspect: usvg::AspectRatio::default(),
        },
        root: root_svg_node,
    }
}
