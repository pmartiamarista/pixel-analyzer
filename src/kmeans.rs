use crate::error::AnalyzerError;
use crate::types::{Cluster, LabColor};

pub fn kmeans_plus_plus(
    pixels: &[LabColor],
    k: usize,
    epsilon: f32,
    max_iters: u32,
) -> Result<Vec<Cluster>, AnalyzerError> {
    if pixels.len() < k {
        return Err(AnalyzerError::InsufficientPixels);
    }

    let mut centroids = init_centroids(pixels, k);

    for _ in 0..max_iters {
        let assignments = assign(pixels, &centroids);

        let new_centroids = update_centroids(&assignments, pixels, &centroids);

        let max_shift = centroids
            .iter()
            .zip(new_centroids.iter())
            .map(|(old, new)| old.delta_e(*new))
            .fold(0.0_f32, f32::max);

        centroids = new_centroids;

        if max_shift < epsilon {
            break;
        }
    }

    let assignments = assign(pixels, &centroids);
    let clusters = build_clusters(assignments, pixels, &centroids);

    Ok(clusters)
}

fn init_centroids(pixels: &[LabColor], k: usize) -> Vec<LabColor> {
    let mut rng = SimpleLcg::new(pixels[0].l.to_bits() as u64 ^ 0xDEAD_BEEF);
    let mut centroids: Vec<LabColor> = Vec::with_capacity(k);

    centroids.push(pixels[rng.next_usize() % pixels.len()]);

    for _ in 1..k {
        let distances: Vec<f32> = pixels
            .iter()
            .map(|px| {
                centroids
                    .iter()
                    .map(|c| px.distance_sq(*c))
                    .fold(f32::MAX, f32::min)
            })
            .collect();

        let total: f32 = distances.iter().sum();
        let mut threshold = rng.next_f32() * total;
        let mut chosen = pixels.len() - 1;

        for (i, &d) in distances.iter().enumerate() {
            threshold -= d;
            if threshold <= 0.0 {
                chosen = i;
                break;
            }
        }

        centroids.push(pixels[chosen]);
    }

    centroids
}

fn assign(pixels: &[LabColor], centroids: &[LabColor]) -> Vec<usize> {
    pixels
        .iter()
        .map(|px| {
            centroids
                .iter()
                .enumerate()
                .map(|(i, c)| (i, px.distance_sq(*c)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap_or(0)
        })
        .collect()
}

fn update_centroids(assignments: &[usize], pixels: &[LabColor], old: &[LabColor]) -> Vec<LabColor> {
    let k = old.len();
    let mut sums = vec![(0.0_f32, 0.0_f32, 0.0_f32); k];
    let mut counts = vec![0usize; k];

    for (&cluster_idx, px) in assignments.iter().zip(pixels.iter()) {
        sums[cluster_idx].0 += px.l;
        sums[cluster_idx].1 += px.a;
        sums[cluster_idx].2 += px.b;
        counts[cluster_idx] += 1;
    }

    sums.iter()
        .zip(counts.iter())
        .enumerate()
        .map(|(i, ((sl, sa, sb), &count))| {
            if count == 0 {
                old[i]
            } else {
                let n = count as f32;
                LabColor {
                    l: sl / n,
                    a: sa / n,
                    b: sb / n,
                }
            }
        })
        .collect()
}

fn build_clusters(
    assignments: Vec<usize>,
    pixels: &[LabColor],
    centroids: &[LabColor],
) -> Vec<Cluster> {
    let k = centroids.len();
    let total_pixels = pixels.len();
    let mut counts = vec![0usize; k];

    for &idx in &assignments {
        counts[idx] += 1;
    }

    let mut clusters: Vec<Cluster> = centroids
        .iter()
        .enumerate()
        .map(|(i, &c)| Cluster {
            centroid: c,
            pixel_count: counts[i],
            total_pixels,
        })
        .collect();

    clusters.sort_by_key(|c| std::cmp::Reverse(c.pixel_count));
    clusters
}

struct SimpleLcg(u64);

impl SimpleLcg {
    fn new(seed: u64) -> Self {
        Self(seed ^ 0x6C62272E_07BB0142)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 11) as f32 / (1u64 << 53) as f32
    }
}
