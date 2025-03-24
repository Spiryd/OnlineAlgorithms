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
    pub fn new(dist_type: DistributionType) -> Self {
        let weights = match dist_type {
            DistributionType::Uniform => vec![1.0; 100],
            DistributionType::Harmonic => (1..=100).map(|i| 1.0 / i as f64).collect(),
            DistributionType::DoublyHarmonic => (1..=100).map(|i| 1.0 / (i * i) as f64).collect(),
            DistributionType::Geometric => {
                let mut weights = Vec::new();
                let mut current_weight = 1.0; // Start with 1/2^0 = 1
                for _ in 0..99 {
                    weights.push(current_weight);
                    current_weight /= 2.0; // Divide by 2 to get the next weight
                }
                weights.push(current_weight); // Add the last weight for Pr[X=100]
                weights
            }
        };

        Self {
            weights,
            rng: rand::rng(),
        }
    }

    pub fn sample(&mut self) -> u32 {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        (dist.sample(&mut self.rng) + 1) as u32 // +1 to map from 0-based index to [1..=100]
    }
}
