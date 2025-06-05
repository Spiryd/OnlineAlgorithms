use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distr::Uniform;
use rand::{prelude::*, rng};
use rayon::prelude::*;

// Number of pages/nodes
const NODES: usize = 64;
// Number of requests per single simulation run
const REQUESTS: usize = 65_536;
// Replication thresholds to test
const DS: [u64; 5] = [16, 32, 64, 128, 256];
// Write‐probabilities to test
const PS: [f64; 6] = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];
// How many independent runs per (D, p) pair
const RUNS: usize = 10_000;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CounterState {
    Normal,
    Waiting,
}

pub enum Request {
    Read(usize),
    Write(usize),
}

impl Request {
    /// Flip a biased coin (probability `p` of Write, else Read).
    pub fn new_random_request(p: f64, page: usize) -> Self {
        if rand::random_bool(p) {
            Request::Write(page)
        } else {
            Request::Read(page)
        }
    }
}

pub struct PageAllocation {
    counts: [(u64, CounterState); NODES],
    threshold: u64,
    copies: HashSet<usize>,
    max_copies: u8,
}

impl PageAllocation {
    /// Start with exactly one replica at page 0 (in Waiting state).
    pub fn new(threshold: u64) -> Self {
        let mut copies = HashSet::new();
        copies.insert(0);
        let mut counts = [(0, CounterState::Normal); NODES];
        counts[0].1 = CounterState::Waiting;
        Self {
            counts,
            threshold,
            copies,
            max_copies: 1,
        }
    }

    /// Process a single Read/Write request, returning the cost of that operation.
    pub fn process_request(&mut self, request: &Request) -> u64 {
        match request {
            Request::Read(page) => self.process_read(*page),
            Request::Write(page) => self.process_write(*page),
        }
    }

    /// Return the maximum replication degree seen in this run.
    pub fn max_copies(&self) -> u8 {
        self.max_copies
    }

    /// Handle a Read(page):
    ///  - If no copy exists, cost += 1 (miss), bump counter if < threshold.
    ///  - If counter == threshold, replicate (cost += threshold)
    fn process_read(&mut self, page: usize) -> u64 {
        let mut cost = 0;

        if !self.copies.contains(&page) {
            // Read miss
            cost += 1;
            if self.counts[page].0 < self.threshold {
                self.counts[page].0 += 1;
            }
        }

        if self.counts[page].0 == self.threshold {
            cost += self.add_copy(page);
        }

        cost
    }

    /// Handle a Write(page):
    ///  - If page is already a replica: cost += (current_copies - 1).
    ///  - Otherwise: cost += current_copies. If exactly one replica is in Waiting, bump counter if < threshold.
    ///  - If counter == threshold, replicate (cost += threshold).
    ///  - Finally, “decay” every other page’s counter and possibly evict.
    fn process_write(&mut self, page: usize) -> u64 {
        let idx = page as usize;
        let mut cost = 0;
        let current_copies = self.copies.len() as u64;

        if self.copies.contains(&page) {
            // Write to an existing replica: propagate to all other copies
            cost += current_copies.saturating_sub(1);
        } else {
            // Write to non-replica: propagate to every existing copy
            cost += current_copies;

            // If there's exactly one replica, bump this page's counter if allowed
            if current_copies == 1
                && self.counts[idx].0 < self.threshold
                && self
                    .counts
                    .iter()
                    .any(|&(_, st)| st == CounterState::Waiting)
            {
                self.counts[idx].0 += 1;
            }
        }

        if self.counts[idx].0 == self.threshold {
            cost += self.add_copy(page);
        }

        // Decay step: for every other page, if it's a replica with counter > 0, decrement.
        // If that counter hits 0 in Waiting, evict (unless it's the last copy).
        for other in 0..NODES {
            if other as usize != page {
                self.process_write_by_another_page(other);
            }
        }

        cost
    }

    /// Called during each write’s decay step on every other page.
    ///  - If this page’s counter > 0 and it is a replica, decrement by 1.
    ///  - If the new counter is 0 and it is a replica:
    ///    • If more than one replica remains, remove it & set state→Normal.
    ///    • Else (sole copy), set state→Waiting.
    fn process_write_by_another_page(&mut self, page: usize) {
        if self.counts[page].0 > 0 && self.copies.contains(&(page as usize)) {
            self.counts[page].0 -= 1;
        }

        if self.counts[page].0 == 0 {
            if self.copies.contains(&(page as usize)) {
                if self.copies.len() > 1 {
                    // Evict this replica
                    self.copies.remove(&(page as usize));
                    self.counts[page].1 = CounterState::Normal;
                } else {
                    // Keep the sole remaining replica in Waiting
                    self.counts[page].1 = CounterState::Waiting;
                }
            }
        }
    }

    /// Insert a new replica on `page`. If successful:
    ///  - Update max_copies
    ///  - Evict exactly one page currently in Waiting, if any
    ///  - Return replication cost = `threshold`. Otherwise return 0.
    fn add_copy(&mut self, page: usize) -> u64 {
        if self.copies.insert(page) {
            let current_count = self.copies.len() as u8;
            if current_count > self.max_copies {
                self.max_copies = current_count;
            }

            // Evict one page that is in Waiting, if found
            if let Some(&victim) = self
                .copies
                .iter()
                .find(|&&p| self.counts[p as usize].1 == CounterState::Waiting)
            {
                self.copies.remove(&victim);
                self.counts[victim as usize].1 = CounterState::Normal;
            }

            // Cost in tokens to replicate
            self.threshold
        } else {
            0
        }
    }
}

/// Simulate exactly `REQUESTS` operations with write‐probability `p` and threshold `d`.
/// Returns a tuple `(sum_of_all_request_costs, peak_replication_degree)`.
fn simulate<R: Rng + ?Sized>(rng: &mut R, p: f64, threshold: u64) -> (f64, usize) {
    let mut alloc = PageAllocation::new(threshold);
    let mut total_cost = 0u64;
    let node_dist = Uniform::new(0, NODES).expect("Uniform distribution should be valid");

    for _ in 0..REQUESTS {
        let page = node_dist.sample(rng) as usize;
        let req = Request::new_random_request(p, page);
        total_cost += alloc.process_request(&req);
    }

    // Return the raw total cost (as f64) and the maximum replication degree
    (total_cost as f64, alloc.max_copies() as usize)
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1) Open CSV and write header
    let file = File::create("results.csv")?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["D", "p", "avg_cost", "avg_max_copies"])?;

    // 2) Set up a progress bar counting all (d, p, run) combinations
    let total_runs = (DS.len() * PS.len() * RUNS) as u64;
    let pb = ProgressBar::new(total_runs);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("=>-"),
    );

    // 3) Build a Vec of all (threshold, p) pairs
    let combos: Vec<(u64, f64)> = DS
        .iter()
        .flat_map(|&d| PS.iter().map(move |&p| (d, p)))
        .collect();

    // 4) For each (d, p), run `RUNS` independent trials in parallel
    let aggregated: Vec<(u64, f64, f64, f64)> = combos
        .par_iter()
        .map(|&(threshold, p)| {
            let mut local_rng = rng();
            let mut sum_total_cost = 0.0;
            let mut sum_maxcopies = 0.0;

            for _ in 0..RUNS {
                let (run_total_cost, run_max_copies) = simulate(&mut local_rng, p, threshold);
                sum_total_cost += run_total_cost;
                sum_maxcopies += run_max_copies as f64;
                pb.inc(1);
            }

            // Compute per‐(d,p) averages over all RUNS
            let avg_total_cost = sum_total_cost / (RUNS as f64);
            let avg_max_copies = sum_maxcopies / (RUNS as f64);
            (threshold, p, avg_total_cost, avg_max_copies)
        })
        .collect();

    // 5) Write each (d, p, avg_total_cost, avg_max_copies) to CSV
    for (d, p, avg_cost, avg_max) in aggregated {
        wtr.write_record(&[
            d.to_string(),
            format!("{:.2}", p),
            format!("{:.2}", avg_cost),
            format!("{:.2}", avg_max),
        ])?;
    }

    wtr.flush()?;
    pb.finish_with_message("Simulation complete");
    println!("Results written to results.csv");
    Ok(())
}
