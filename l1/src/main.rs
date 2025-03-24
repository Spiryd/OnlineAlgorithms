mod linked_list;
use linked_list::{LinkedList, ListType};

mod sampler;
use sampler::{DistributionType, RandomSampler};

use std::collections::HashMap;

const SAMPLE_SIZE: usize = 1;

fn main() {
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
            for n in [100, 500, 1000, 5000, 10_000, 50_000, 100_000] {
                println!(
                    "List type: {:?}, Distribution type: {:?}, n: {}",
                    list_type, distribution_type, n
                );
                for _ in 0..SAMPLE_SIZE {
                    let mut list = LinkedList::new(list_type.clone());
                    for _ in 0..n {
                        list.access(sampler.sample());
                    }
                }
            }
        }
    }
}
