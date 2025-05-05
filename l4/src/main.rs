// src/main.rs
use indicatif::{ProgressBar, ProgressStyle};
use rand::{distr::weighted::WeightedIndex, prelude::Distribution, prelude::ThreadRng, rng, Rng};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

// ——— Metric & Algorithms ——————————————————————————————————————

#[derive(Clone, Copy, Debug)]
pub enum GraphStructure {
    Hypercube,
    Torus,
}

impl GraphStructure {
    /// Shortest‐path distance on 64‐node Hypercube or 4×4×4 Torus.
    pub fn distance(&self, a: usize, b: usize) -> usize {
        const N: usize = 64;
        assert!(a < N && b < N);
        match self {
            GraphStructure::Hypercube => (a ^ b).count_ones() as usize,
            GraphStructure::Torus => {
                // decode 0..63 into (x,y,z) in 0..3
                let coord = |u, d| match d {
                    0 =>  u       % 4,
                    1 => (u / 4)  % 4,
                    2 => (u / 16) % 4,
                    _ => unreachable!(),
                };
                (0..3).map(|d| {
                    let ca = coord(a, d);
                    let cb = coord(b, d);
                    let delta = ca.abs_diff(cb);
                    std::cmp::min(delta, 4 - delta)
                }).sum()
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MigrationType {
    MoveToMin,
    CoinFlip,
}

pub struct PageMigration {
    page: usize,
    d: usize,
    metric: GraphStructure,
    policy: MigrationType,
    buffer: Vec<usize>, // only used for MoveToMin
}

impl PageMigration {
    pub fn new(start: usize, d: usize, metric: GraphStructure, policy: MigrationType) -> Self {
        assert!(start < 64);
        PageMigration {
            page: start,
            d,
            metric,
            policy,
            buffer: Vec::with_capacity(d),
        }
    }

    /// Serve one request; return access + (optional) migration cost.
    pub fn on_request(&mut self, req: usize, rng: &mut ThreadRng) -> usize {
        assert!(req < 64);
        let dist = self.metric.distance(self.page, req);
        let mut cost = dist;

        match self.policy {
            MigrationType::MoveToMin => {
                self.buffer.push(req);
                if self.buffer.len() == self.d {
                    // choose m minimizing ∑d(m, vi)
                    let best = (0..64)
                        .min_by_key(|&cand| {
                            self.buffer.iter()
                                .map(|&v| self.metric.distance(cand, v))
                                .sum::<usize>()
                        })
                        .unwrap();
                    let mig = self.metric.distance(self.page, best);
                    cost += self.d * mig;
                    self.page = best;
                    self.buffer.clear();
                }
            }
            MigrationType::CoinFlip => {
                let p = 1.0 / (2.0 * (self.d as f64));
                if rng.random_bool(p) {
                    cost += self.d * dist;
                    self.page = req;
                }
            }
        }

        cost
    }
}

// ——— Main Simulation ——————————————————————————————————————

fn uniform_weights(n: usize) -> Vec<f64> {
    vec![1.0 / (n as f64); n]
}
fn harmonic_weights(n: usize) -> Vec<f64> {
    let h: f64 = (1..=n).map(|i| 1.0 / (i as f64)).sum();
    (1..=n).map(|i| (1.0 / (i as f64)) / h).collect()
}
fn biharmonic_weights(n: usize) -> Vec<f64> {
    let h2: f64 = (1..=n).map(|i| 1.0 / ((i*i) as f64)).sum();
    (1..=n).map(|i| (1.0 / ((i*i) as f64)) / h2).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // parameters
    let n = 64;
    let req_len = 65_536;
    let ds = [16, 32, 64, 128, 256];
    let graphs = [
        (GraphStructure::Torus,     "Torus 4x4x4"),
        (GraphStructure::Hypercube, "Hypercube 6D"),
    ];
    let dists = [
        (uniform_weights(n),   "Uniform"),
        (harmonic_weights(n),  "Harmonic"),
        (biharmonic_weights(n),"Biharmonic"),
    ];
    let algos = [
        (MigrationType::MoveToMin, "MoveToMin"),
        (MigrationType::CoinFlip,  "CoinFlip"),
    ];
    let iterations = 1_000;

    // build a flat list of tasks
    struct Task {
        metric: GraphStructure,
        gname: &'static str,
        weights: Vec<f64>,
        dname: &'static str,
        d: usize,
        policy: MigrationType,
        pname: &'static str,
    }
    let mut tasks = Vec::with_capacity(
        graphs.len() * dists.len() * ds.len() * algos.len()
    );
    for &(metric, gname) in &graphs {
        for (weights, dname) in &dists {
            for &d in &ds {
                for &(policy, pname) in &algos {
                    tasks.push(Task {
                        metric,
                        gname,
                        weights: weights.clone(),
                        dname,
                        d,
                        policy,
                        pname,
                    });
                }
            }
        }
    }

    // progress bar
    let total = tasks.len() * iterations;
    let pb = Arc::new(ProgressBar::new(total as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    // run all tasks in parallel
    let results: Vec<String> = tasks
    .into_par_iter()
    .flat_map_iter(|task| {
        // prepare RNG / sampler
        let mut rng = rng();
        let sampler = WeightedIndex::new(&task.weights).unwrap();

        // now return a _normal_ iterator of Strings
        (0..iterations).map({
            let pb = Arc::clone(&pb);
            move |_| {
                // build sim, run it, format your CSV line
                let mut sim = PageMigration::new(0, task.d, task.metric, task.policy);
                let reqs: Vec<usize> = (0..req_len)
                    .map(|_| sampler.sample(&mut rng))
                    .collect();
                let cost: usize = reqs.into_iter()
                    .map(|r| sim.on_request(r, &mut rng))
                    .sum();
                pb.inc(1);
                format!("{},{},{},{},{}", task.gname, task.dname, task.d, task.pname, cost)
            }
        })
    })
    .collect();


    pb.finish_with_message("Simulation complete!");

    // write CSV
    let f = File::create("results.csv")?;
    let mut w = BufWriter::new(f);
    writeln!(w, "Graph,Distribution,D,Algorithm,Cost")?;
    for line in results {
        writeln!(w, "{}", line)?;
    }
    w.flush()?;

    Ok(())
}
