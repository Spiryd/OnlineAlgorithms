mod cache;
use cache::{CacheManagementStrategy, CacheManager};

mod sampler;
use sampler::{DistributionType, RandomSampler};

use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Write};
use std::sync::Mutex;
use std::time::Instant;

// Number of simulation trials per combination.
const TRIALS: usize = 100;
// Number of page requests per trial.
const NUM_REQUESTS: usize = 100_000;

fn main() -> io::Result<()> {
    // Start the timer to measure the execution time.
    let start_time = Instant::now();

    // Create and open the CSV file for writing.
    let file = File::create("cache_results.csv")?;
    let file = Mutex::new(file);
    writeln!(
        file.lock().unwrap(),
        "n;k;cache_strategy;distribution;avg_cost"
    )?;

    // Define cache strategies as (name, factory function producing a new variant).
    let cache_strategies: Vec<(&str, Box<dyn Fn() -> CacheManagementStrategy + Sync>)> = vec![
        ("FIFO", Box::new(|| CacheManagementStrategy::FIFO)),
        ("FWF", Box::new(|| CacheManagementStrategy::FWF)),
        (
            "LRU",
            Box::new(|| CacheManagementStrategy::LRU(VecDeque::new())),
        ),
        (
            "LFU",
            Box::new(|| CacheManagementStrategy::LFU(HashMap::new())),
        ),
        (
            "RAND",
            Box::new(|| CacheManagementStrategy::RAND(rand::rng())),
        ),
        (
            "RMA",
            Box::new(|| CacheManagementStrategy::RMA(HashMap::new(), rand::rng())),
        ),
    ];

    // Define the distribution types.
    let distribution_types = [
        DistributionType::Uniform,
        DistributionType::Harmonic,
        DistributionType::DoublyHarmonic,
        DistributionType::Geometric,
    ];

    // n is the endpoint (sample pages from 1..=n)
    // k is the cache (page) size.
    (20..=100)
        .step_by(10)
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|n| {
            (n / 10..=n / 5)
                .collect::<Vec<_>>()
                .into_par_iter()
                .for_each(|k| {
                    cache_strategies.par_iter().for_each(
                        |&(strategy_name, ref strategy_factory)| {
                            distribution_types.par_iter().for_each(|&distribution_type| {
                            // Create a RandomSampler for the current distribution, sampling from 1..=n.
                            let mut sampler = RandomSampler::new(distribution_type, n);
                            // Run TRIALS simulation trials.
                            println!(
                                "Running simulation for n={}, k={}, strategy={}, distribution={:?}",
                                n, k, strategy_name, distribution_type
                            );
                            let mut payload = String::new();
                            let mut total_cost: usize;
                            for _ in 0..TRIALS {
                                total_cost = 0;
                                // Create a fresh cache manager with capacity k.
                                let mut cache = CacheManager::new(k, strategy_factory());
                                // Simulate NUM_REQUESTS page accesses.
                                for _ in 0..NUM_REQUESTS {
                                    let page = sampler.sample() as usize;
                                    total_cost += cache.access(page);
                                }
                                payload.push_str(&format!(
                                    "{};{};{};{:?};{}\n",
                                    n, k, strategy_name, distribution_type, total_cost as f64 / NUM_REQUESTS as f64
                                ));
                            }
                            let mut file = file.lock().unwrap();
                            write!(file, "{}", payload).expect("Failed to write to file");
                        });
                        },
                    );
                });
        });
    let elapsed_time = start_time.elapsed();
    println!(
        "Total processing time: {:.2?} seconds",
        elapsed_time.as_secs_f64()
    );

    Ok(())
}
