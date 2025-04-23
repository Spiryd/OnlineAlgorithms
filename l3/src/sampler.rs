use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum DistributionType {
    Uniform,
    Harmonic,
    DoublyHarmonic,
    Geometric,
}

#[derive(Debug)]
pub struct RandomSampler {
    weights: Vec<f64>,
    rng: ThreadRng,
}

impl RandomSampler {
    /// Creates a new RandomSampler for the given distribution type,
    /// sampling from the range {1..=endpoint}
    pub fn new(dist_type: DistributionType, endpoint: usize) -> Self {
        let weights = match dist_type {
            DistributionType::Uniform => vec![1.0; endpoint],
            DistributionType::Harmonic => (1..=endpoint).map(|i| 1.0 / i as f64).collect(),
            DistributionType::DoublyHarmonic => {
                (1..=endpoint).map(|i| 1.0 / ((i * i) as f64)).collect()
            }
            DistributionType::Geometric => {
                let mut weights = Vec::with_capacity(endpoint);
                let mut current_weight = 1.0; // Start with weight 1 for 1
                for _ in 0..endpoint {
                    weights.push(current_weight);
                    current_weight /= 2.0; // Next weight is half the previous
                }
                weights
            }
        };

        Self {
            weights,
            rng: rand::rng(),
        }
    }

    /// Samples a value from {1, 2, …, endpoint} using the specified weighted distribution.
    pub fn sample(&mut self) -> u32 {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        (dist.sample(&mut self.rng) + 1) as u32 // +1 to shift from 0-based to 1-based index
    }
}
