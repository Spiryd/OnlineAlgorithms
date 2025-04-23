mod packing;
mod sampler;

use packing::{BinPackingManager, PackingStrategy};
use rand::Rng;
use sampler::{DistributionType, RandomSampler};

use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use indicatif::ProgressBar;

const SAMPLE_SIZE: usize = 100_000;
const TOTAL_ITEMS: usize = 100;

fn main() -> io::Result<()> {
    // Start the timer to measure total execution time.
    let start_time = Instant::now();

    // Create and open the CSV file for writing.
    let file = File::create("results.csv")?;
    let file = Mutex::new(file); // Wrap the file in a Mutex for synchronized access
    writeln!(
        file.lock().unwrap(),
        "distribution;strategy;experiment;bin_count;item_sum"
    )?;

    // Define the distribution types.
    let distributions = [
        DistributionType::Uniform,
        DistributionType::Harmonic,
        DistributionType::DoublyHarmonic,
        DistributionType::Geometric,
    ];

    // Define the packing strategies as factories (closures).
    let strategy_factories: Vec<(&str, Box<dyn Fn() -> PackingStrategy + Sync>)> = vec![
        ("NextFit", Box::new(|| PackingStrategy::NextFit)),
        ("RandomFit", Box::new(|| PackingStrategy::RandomFit(rand::rng()))),
        ("FirstFit", Box::new(|| PackingStrategy::FirstFit)),
        ("BestFit", Box::new(|| PackingStrategy::BestFit)),
        ("WorstFit", Box::new(|| PackingStrategy::WorstFit)),
    ];

    // Calculate the total number of iterations for the progress bar.
    let total_iterations = distributions.len() * strategy_factories.len();
    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(total_iterations as u64)));

    // Use parallel iterators for multithreading.
    distributions.par_iter().for_each(|distribution| {
        strategy_factories.par_iter().for_each(|(strategy_name, strategy_factory)| {
            let mut results = Vec::new();
            for experiment in 0..SAMPLE_SIZE {
                // Create a sampler for the current distribution.
                let mut sampler = RandomSampler::new(*distribution, 10);
                let mut manager = BinPackingManager::new(strategy_factory());
                let mut rng: rand::prelude::ThreadRng = rand::rng(); // Thread-local RNG for safety.
                let mut total_items = 0;
                let mut item_sum = 0.0;

                // Add items until the total reaches TOTAL_ITEMS.
                while total_items < TOTAL_ITEMS {
                    let k = sampler.sample(); // Random k from 1 to 10.
                    for _ in 0..k {
                        if total_items >= TOTAL_ITEMS {
                            break;
                        }
                        let item: f64 = rng.random_range(0.0..=1.0); // Random item weight between 0 and 1.
                        manager.add_item(item);
                        item_sum += item; // Accumulate the sum of items.
                        total_items += 1;
                    }
                }

                // Calculate the number of bins used.
                let bin_count = manager.bins().len();

                // Format the result as a CSV row.
                results.push(format!(
                    "{:?};{};{};{};{:.2}",
                    distribution, strategy_name, experiment, bin_count, item_sum
                ));
            }

            // Increment the progress bar.
            let progress_bar = progress_bar.clone();
            let pb = progress_bar.lock().unwrap();
            pb.inc(1);
            // Write results to the CSV file in a thread-safe manner.
            let mut file = file.lock().unwrap();
            for result in results {
                writeln!(file, "{}", result).expect("Failed to write to file");
            }
        });
    });

    // Finish the progress bar.
    progress_bar.lock().unwrap().finish();

    // Print the total elapsed time.
    let elapsed_time = start_time.elapsed();
    println!("Total time elapsed: {:.2?}", elapsed_time);

    Ok(())
}