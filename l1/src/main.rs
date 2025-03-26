mod linked_list;
use linked_list::{LinkedList, ListType};

mod sampler;
use sampler::{DistributionType, RandomSampler};

use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::sync::Mutex;

const SAMPLE_SIZE: usize = 1000;

fn main() -> io::Result<()> {
    let file = File::create("l1.csv")?;
    let file = Mutex::new(file); // Wrap the file in a Mutex for synchronized access
    writeln!(file.lock().unwrap(), "n;list_type;distribution;total_cost")?;

    let ns = [100, 500, 1000, 5000, 10_000, 50_000, 100_000];
    let list_types = [
        ListType::Simple,
        ListType::MoveToFront,
        ListType::Transpose,
        ListType::Count(HashMap::new()),
    ];
    let distribution_types = [
        DistributionType::Uniform,
        DistributionType::Harmonic,
        DistributionType::DoublyHarmonic,
        DistributionType::Geometric,
    ];

    ns.par_iter().for_each(|&n| {
        list_types.par_iter().for_each(|list_type| {
            distribution_types.par_iter().for_each(|distribution_type| {
                let mut sampler = RandomSampler::new(*distribution_type);
                println!(
                    "List type: {:?}, Distribution type: {:?}, n: {}",
                    list_type, distribution_type, n
                );
                let mut results = Vec::new();
                for _ in 0..SAMPLE_SIZE {
                    let mut list = LinkedList::new(list_type.clone());
                    let mut total_cost = 0;
                    for _ in 0..n {
                        total_cost += list.access(sampler.sample());
                    }
                    results.push(format!(
                        "{};{:?};{:?};{}",
                        n, list_type, distribution_type, total_cost
                    ));
                }
                let mut file = file.lock().unwrap(); // Lock the file for writing
                for result in results {
                    writeln!(file, "{}", result).expect("Failed to write to file");
                }
            });
        });
    });

    Ok(())
}
