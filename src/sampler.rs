use crate::config::Quality;
use crate::types::RgbColor;

const MAX_DIM: [u32; 3] = [128, 256, 512];

const GRID_N: usize = 32;

pub fn sample_pixels(pixels: &[u8], width: u32, height: u32, quality: Quality) -> Vec<RgbColor> {
    let max_dim = MAX_DIM[quality as usize];

    let (ds_pixels, ds_width, ds_height) = downsample(pixels, width, height, max_dim);

    stratified_sample(&ds_pixels, ds_width, ds_height, quality.sample_fraction())
}

fn downsample(src: &[u8], src_w: u32, src_h: u32, max_dim: u32) -> (Vec<u8>, u32, u32) {
    if src_w <= max_dim && src_h <= max_dim {
        return (src.to_vec(), src_w, src_h);
    }

    let scale = (max_dim as f32) / (src_w.max(src_h) as f32);
    let dst_w = ((src_w as f32 * scale).round() as u32).max(1);
    let dst_h = ((src_h as f32 * scale).round() as u32).max(1);

    let mut dst = Vec::with_capacity((dst_w * dst_h * 4) as usize);

    for dy in 0..dst_h {
        for dx in 0..dst_w {
            let sx = ((dx as f32 / dst_w as f32) * src_w as f32) as u32;
            let sy = ((dy as f32 / dst_h as f32) * src_h as f32) as u32;
            let idx = ((sy * src_w + sx) * 4) as usize;
            dst.push(src[idx]);
            dst.push(src[idx + 1]);
            dst.push(src[idx + 2]);
            dst.push(src[idx + 3]);
        }
    }

    (dst, dst_w, dst_h)
}

fn stratified_sample(pixels: &[u8], width: u32, height: u32, fraction: f32) -> Vec<RgbColor> {
    let w = width as usize;
    let h = height as usize;
    let grid_n = GRID_N;

    let total_opaque_est = (w * h) as f32 * fraction;
    let mut result = Vec::with_capacity(total_opaque_est as usize);

    for gy in 0..grid_n {
        for gx in 0..grid_n {
            let x0 = (gx * w) / grid_n;
            let x1 = ((gx + 1) * w) / grid_n;
            let y0 = (gy * h) / grid_n;
            let y1 = ((gy + 1) * h) / grid_n;

            let cell_pixels: Vec<RgbColor> = collect_cell(pixels, w, x0, x1, y0, y1);
            if cell_pixels.is_empty() {
                continue;
            }

            let take = ((cell_pixels.len() as f32 * fraction).ceil() as usize).max(1);
            let step = (cell_pixels.len() / take).max(1);

            for (i, px) in cell_pixels.iter().enumerate() {
                if i % step == 0 {
                    result.push(*px);
                }
            }
        }
    }

    result
}

fn collect_cell(
    pixels: &[u8],
    width: usize,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
) -> Vec<RgbColor> {
    let mut out = Vec::new();
    for y in y0..y1 {
        for x in x0..x1 {
            let idx = (y * width + x) * 4;
            let alpha = pixels[idx + 3];
            if alpha >= 128 {
                out.push(RgbColor {
                    r: pixels[idx],
                    g: pixels[idx + 1],
                    b: pixels[idx + 2],
                });
            }
        }
    }
    out
}
