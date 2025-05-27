use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distr::Uniform;
use rand::prelude::*;
use rand::rng;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

const NODES: usize = 64;
const REQUESTS: usize = 65_536;
const DS: [usize; 5] = [16, 32, 64, 128, 256];
const PS: [f64; 6] = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];
const RUNS: usize = 10_000;

/// Simulate the COUNT algorithm over `REQUESTS` operations
/// with replication threshold `threshold` and write probability `p`.
fn simulate<R: Rng + ?Sized>(rng: &mut R, p: f64, threshold: usize) -> (f64, usize) {
    let mut copies = HashSet::new();
    copies.insert(0);

    let mut counters = vec![0usize; NODES];
    let mut total_cost = 0u64;
    let mut max_copies = copies.len();

    // uniform distribution over node IDs 0..NODES
    let node_dist = Uniform::new(0, NODES).expect("Invalid node distribution");

    for _ in 0..REQUESTS {
        let v = node_dist.sample(rng);
        let is_write = rng.random_bool(p);
        let c = copies.len() as u64;

        if is_write {
            // write cost: propagate to existing replicas
            let req_cost = if copies.contains(&v) { c - 1 } else { c };
            total_cost += req_cost;
            counters[v] += 1;
            // replicate if write threshold exceeded
            if counters[v] > threshold {
                if copies.insert(v) {
                    total_cost += threshold as u64;
                }
                counters[v] = 0;
                max_copies = max_copies.max(copies.len());
            }
        } else {
            // read cost: hit = 0, miss = 1
            let req_cost = if copies.contains(&v) { 0 } else { 1 };
            total_cost += req_cost;
        }
    }

    let avg_cost = total_cost as f64 / REQUESTS as f64;
    (avg_cost, max_copies)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare CSV writer
    let file = File::create("results.csv")?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["D", "p", "avg_cost", "avg_max_copies"])?;

    // Set up a fine-grained progress bar counting each simulation run
    let total_runs = (DS.len() * PS.len() * RUNS) as u64;
    let pb = ProgressBar::new(total_runs);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("=>-")
    );

    // Parallel simulation: for each (D, p) perform RUNS independent runs
    let mut aggregated: Vec<_> = DS.iter()
        .flat_map(|&d| PS.iter().map(move |&p| (d, p)))
        .collect::<Vec<(usize, f64)>>()
        .par_iter()
        .map(|&(d, p)| {
            let mut rng = rng();
            let mut sum_cost = 0.0;
            let mut sum_max = 0;
            for _ in 0..RUNS {
                let (cost, m) = simulate(&mut rng, p, d);
                sum_cost += cost;
                sum_max += m;
                pb.inc(1);
            }
            // compute averages
            let mean_cost = sum_cost / RUNS as f64;
            let mean_max = sum_max as f64 / RUNS as f64;
            (d, p, mean_cost, mean_max)
        })
        .collect();

    // Write aggregated results to CSV
    for (d, p, mean_cost, mean_max) in aggregated.drain(..) {
        wtr.write_record(&[
            d.to_string(),
            format!("{:.2}", p),
            format!("{:.4}", mean_cost),
            format!("{:.2}", mean_max),
        ])?;
    }

    // Finalize
    wtr.flush()?;
    pb.finish_with_message("Simulation complete");
    println!("Results written to results.csv");
    Ok(())
}
