use crate::common::Rectangle;
use crate::model::Color;

use image::GenericImageView;
use itertools::Itertools;
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Chunk, Filter, Finish};
use pdf_writer::{Content, Name, Rect, Ref};

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

pub(crate) struct PdfBuilder {
    pdf: pdf_writer::Pdf,
    page_ids: Vec<Ref>,
    alloc_ref: Ref,
    page_tree_id: Ref,
}

impl PdfBuilder {
    pub fn new(n_pages: u32) -> Self {
        let mut pdf = pdf_writer::Pdf::new();

        let mut alloc_ref = Ref::new(1);

        let catalog_id = alloc_ref.bump();
        let page_tree_id = alloc_ref.bump();

        pdf.catalog(catalog_id).pages(page_tree_id);

        let page_ids: Vec<Ref> = (0..n_pages).map(|_| alloc_ref.bump()).collect();
        pdf.pages(page_tree_id)
            .kids(page_ids.iter().copied())
            .count(page_ids.len() as i32);

        PdfBuilder {
            pdf,
            page_ids,
            alloc_ref,
            page_tree_id,
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk, chunk_ref: Ref) -> Ref {
        let mut map = HashMap::<Ref, Ref>::new();
        let chunk = chunk.renumber(|r| *map.entry(r).or_insert_with(|| self.alloc_ref.bump()));
        self.pdf.extend(&chunk);
        *map.get(&chunk_ref).unwrap()
    }

    pub fn add_chunk_direct(&mut self, chunk: Chunk) {
        self.pdf.extend(&chunk);
    }

    pub fn ref_bump(&mut self) -> Ref {
        self.alloc_ref.bump()
    }

    pub fn add_page_from_svg(
        &mut self,
        page_idx: usize,
        width: f32,
        height: f32,
        bg_color: Color,
        refs: &[(Rectangle, Ref)],
    ) {
        /*let (svg_chunk, svg_id) =
            svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default(), font_db);
        let svg_id = self.add_chunk(svg_chunk, svg_id);*/

        let page_id = self.page_ids[page_idx];
        /*let name_str = format!("S{}", page_idx);
        let svg_name = Name(name_str.as_bytes());*/

        let names = refs
            .iter()
            .enumerate()
            .map(|(i, _)| format!("o{}", i))
            .collect_vec();

        let content_id = self.alloc_ref.bump();
        let mut page = self.pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, width, height));
        page.parent(self.page_tree_id);
        page.contents(content_id);

        let mut resources = page.resources();
        let mut objects = resources.x_objects();
        for (name, (_, rf)) in names.iter().zip(refs) {
            objects.pair(Name(name.as_bytes()), rf);
        }
        objects.finish();
        resources.finish();
        page.finish();

        let mut content = Content::new();

        content.save_state();
        let (r, g, b) = bg_color.as_3f32();
        content.set_fill_rgb(r, g, b);
        content.rect(0.0, 0.0, width, height);
        content.fill_nonzero();
        content.restore_state();
        // content.transform([width, 0.0, 0.0, height, 0.0, 0.0]);
        for (name, (rect, _)) in names.iter().zip(refs) {
            content
                .save_state()
                .transform([
                    rect.width,
                    0.0,
                    0.0,
                    rect.height,
                    rect.x,
                    height - rect.height - rect.y,
                ])
                .x_object(Name(name.as_bytes()))
                .restore_state();
        }
        self.pdf.stream(content_id, &content.finish());
    }

    pub fn write(self, path: &Path) -> crate::Result<()> {
        std::fs::write(path, self.pdf.finish())?;
        Ok(())
    }
}

pub fn image_to_pdf_chunk(
    image_format: image::ImageFormat,
    data: &[u8],
    image_ref: Ref,
    mask_ref: Ref,
) -> (Chunk, Ref) {
    let dynamic = image::load_from_memory_with_format(data, image_format).unwrap();

    let (filter, encoded, mask) = match image_format {
        image::ImageFormat::Jpeg => {
            assert_eq!(dynamic.color(), image::ColorType::Rgb8);
            (Filter::DctDecode, Cow::Borrowed(data), None)
        }

        image::ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, Cow::Owned(encoded), mask)
        }
        _ => panic!("unsupported image format"),
    };

    let mut chunk = Chunk::new();
    let mut image = chunk.image_xobject(image_ref, &encoded);
    image.filter(filter);
    image.width(dynamic.width() as i32);
    image.height(dynamic.height() as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if mask.is_some() {
        image.s_mask(mask_ref);
    }
    image.finish();

    if let Some(encoded) = &mask {
        let mut s_mask = chunk.image_xobject(mask_ref, encoded);
        s_mask.filter(filter);
        s_mask.width(dynamic.width() as i32);
        s_mask.height(dynamic.height() as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
        s_mask.finish();
    }
    (chunk, image_ref)
}
