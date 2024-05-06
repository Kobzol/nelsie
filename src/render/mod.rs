mod canvas;
mod canvas_pdf;
mod canvas_svg;
mod counters;
mod image;
mod layout;
mod pagebuilder;
mod pathbuilder;
mod paths;
mod pdf;
mod rendering;
mod text;

use crate::model::{FontData, Resources, Slide, SlideDeck, SlideId};
pub(crate) use pdf::PdfBuilder;

use crate::common::Step;
use crate::render::counters::{compute_counters, CountersMap};
use crate::render::pagebuilder::PageBuilder;
use crate::render::rendering::render_to_canvas;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use std::path::Path;
use std::sync::Arc;

pub(crate) struct RenderConfig<'a> {
    pub resources: &'a Resources,
    pub slide: &'a Slide,
    pub slide_id: SlideId,
    pub step: Step,
    pub default_font: &'a Arc<FontData>,
    pub counter_values: &'a CountersMap<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum OutputFormat {
    Pdf,
    Svg,
    Png,
}

pub(crate) struct OutputConfig<'a> {
    pub path: Option<&'a Path>,
    pub format: OutputFormat,
}

pub(crate) enum VerboseLevel {
    Silent,
    Normal,
    Full,
}

impl VerboseLevel {
    pub fn is_full(&self) -> bool {
        match self {
            VerboseLevel::Silent | VerboseLevel::Normal => false,
            VerboseLevel::Full => true,
        }
    }
    pub fn is_normal_or_more(&self) -> bool {
        match self {
            VerboseLevel::Silent => false,
            VerboseLevel::Normal | VerboseLevel::Full => true,
        }
    }
}

fn render_slide(
    resources: &Resources,
    builder: &PageBuilder,
    slide_id: SlideId,
    slide: &Slide,
    default_font: &Arc<FontData>,
    counter_values: &CountersMap,
) -> crate::Result<()> {
    log::debug!("Rendering slide {}", slide_id);
    (1..=slide.n_steps)
        .into_par_iter()
        .map(|step| {
            let render_cfg = RenderConfig {
                resources,
                slide,
                slide_id,
                step,
                default_font,
                counter_values,
            };
            let canvas = render_to_canvas(&render_cfg);
            let counter = render_cfg.counter_values.get("global").unwrap();
            let page_idx = counter
                .indices
                .get(&(render_cfg.slide_id, render_cfg.step))
                .unwrap()
                .page_idx;
            builder.add_page(slide_id, step, page_idx, canvas, render_cfg.resources)
        })
        .collect::<crate::Result<Vec<()>>>()?;
    Ok(())
}

pub(crate) fn render_slide_deck(
    slide_deck: &SlideDeck,
    resources: &Resources,
    output_cfg: &OutputConfig,
    verbose_level: VerboseLevel,
    n_threads: Option<usize>,
) -> crate::Result<Vec<(usize, usize, Vec<u8>)>> {
    let start_time = std::time::Instant::now();
    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(n_threads) = n_threads {
        thread_pool_builder = thread_pool_builder.num_threads(n_threads);
    }
    let thread_pool = thread_pool_builder.build().unwrap();
    let result = thread_pool.install(|| {
        if verbose_level.is_full() {
            println!(
                "Slides construction: {:.2}s",
                (start_time - slide_deck.creation_time).as_secs_f32()
            );
        }

        let counter_values = compute_counters(slide_deck);
        let global_counter = counter_values.get("global").unwrap();

        let progress_bar = if verbose_level.is_normal_or_more() {
            Some(indicatif::ProgressBar::new(global_counter.n_pages.into()))
        } else {
            None
        };
        let builder =
            PageBuilder::new(slide_deck, output_cfg, progress_bar, global_counter.n_pages)?;

        slide_deck
            .slides
            .par_iter()
            .enumerate()
            .map(|(slide_idx, slide)| {
                render_slide(
                    resources,
                    &builder,
                    slide_idx as SlideId,
                    slide,
                    &slide_deck.default_font,
                    &counter_values,
                )
            })
            .collect::<crate::Result<Vec<()>>>()?;

        let result_data = builder.finish()?;

        if verbose_level.is_full() {
            let render_end_time = std::time::Instant::now();
            println!(
                "Total rendering time: {:.2}s",
                (render_end_time - start_time).as_secs_f32()
            );
        }
        Ok(result_data)
    });
    if verbose_level.is_full() {
        let render_end_time = std::time::Instant::now();
        println!(
            "Total rendering time: {:.2}s",
            (render_end_time - start_time).as_secs_f32()
        );
    }
    result
}
