mod linked_list;
use linked_list::{LinkedList, ListType};

mod sampler;
use sampler::{DistributionType, RandomSampler};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

const SAMPLE_SIZE: usize = 100;

fn main() -> io::Result<()> {
    let mut file = File::create("l1.csv")?;
    writeln!(file, "n;list_type;distribution;total_cost")?;

    for n in [100, 500, 1000, 5000, 10_000, 50_000, 100_000] {
        for list_type in [
            ListType::Simple,
            ListType::MoveToFront,
            ListType::Transpose,
            ListType::Count(HashMap::new()),
        ] {
            for distribution_type in [
                DistributionType::Uniform,
                DistributionType::Harmonic,
                DistributionType::DoublyHarmonic,
                DistributionType::Geometric,
            ] {
                let mut sampler = RandomSampler::new(distribution_type);
                println!(
                    "List type: {:?}, Distribution type: {:?}, n: {}",
                    list_type, distribution_type, n
                );
                for _ in 0..SAMPLE_SIZE {
                    let mut list = LinkedList::new(list_type.clone());
                    let mut total_cost = 0;
                    for _ in 0..n {
                        total_cost += list.access(sampler.sample());
                    }
                    writeln!(
                        file,
                        "{};{:?};{:?};{}",
                        n, list_type, distribution_type, total_cost
                    )?;
                }
            }
        }
    }

    Ok(())
}
