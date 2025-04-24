use rand::seq::IteratorRandom;
use rand::rngs::ThreadRng;

/// The capacity of each bin.
const BIN_CAPACITY: f64 = 1.0;

/// Enum representing different bin packing strategies.
#[derive(Debug, Clone)]
pub enum PackingStrategy {
    /// Next-Fit strategy: Place the item in the last bin if it fits; otherwise, start a new bin.
    NextFit,
    /// Random-Fit strategy: Place the item in a randomly chosen bin that has enough space.
    RandomFit(ThreadRng),
    /// First-Fit strategy: Place the item in the first bin that has enough space.
    FirstFit,
    /// Best-Fit strategy: Place the item in the bin that leaves the least leftover space.
    BestFit,
    /// Worst-Fit strategy: Place the item in the bin that leaves the most leftover space.
    WorstFit,
}

/// A manager for handling bin packing operations.
#[derive(Debug)]
pub struct BinPackingManager {
    /// The packing strategy to use.
    strategy: PackingStrategy,
    /// The list of bins, where each bin is represented by its current load.
    bins: Vec<f64>,
}

impl BinPackingManager {
    /// Creates a new `BinPackingManager` with the specified packing strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The packing strategy to use.
    pub fn new(strategy: PackingStrategy) -> Self {
        BinPackingManager {
            strategy,
            bins: Vec::new(),
        }
    }

    /// Returns a reference to the current list of bins.
    pub fn bins(&self) -> &[f64] {
        &self.bins
    }

    /// Adds an item to the bins using the specified packing strategy.
    ///
    /// # Arguments
    ///
    /// * `item` - The size of the item to add.
    pub fn add_item(&mut self, item: f64) {
        match self.strategy {
            PackingStrategy::NextFit => self._next_fit(item),
            PackingStrategy::RandomFit(_) => self._random_fit(item),
            PackingStrategy::FirstFit => self._first_fit(item),
            PackingStrategy::BestFit => self._best_fit(item),
            PackingStrategy::WorstFit => self._worst_fit(item),
        }
    }

    /// Implements the Next-Fit strategy.
    fn _next_fit(&mut self, item: f64) {
        if let Some(last) = self.bins.last_mut() {
            if *last + item <= BIN_CAPACITY {
                *last += item;
                return;
            }
        }
        // Start a new bin if the item doesn't fit in the last bin.
        self.bins.push(item);
    }

    /// Implements the Random-Fit strategy.
    fn _random_fit(&mut self, item: f64) {
        if let PackingStrategy::RandomFit(ref mut rng) = self.strategy {
            let candidates = self
                .bins
                .iter_mut()
                .enumerate()
                .filter(|(_, load)| **load + item <= BIN_CAPACITY);
            if let Some((_, load)) = candidates.choose(rng) {
                *load += item;
            } else {
                self.bins.push(item);
            }
        }
    }

    /// Implements the First-Fit strategy.
    fn _first_fit(&mut self, item: f64) {
        for load in &mut self.bins {
            if *load + item <= BIN_CAPACITY {
                *load += item;
                return;
            }
        }
        // Start a new bin if no existing bin can accommodate the item.
        self.bins.push(item);
    }

    /// Implements the Best-Fit strategy.
    fn _best_fit(&mut self, item: f64) {
        let mut best_idx: Option<usize> = None;
        let mut best_leftover = BIN_CAPACITY + 1.0;
        for (i, &load) in self.bins.iter().enumerate() {
            let leftover = BIN_CAPACITY - (load + item);
            if leftover >= 0.0 && leftover < best_leftover {
                best_leftover = leftover;
                best_idx = Some(i);
            }
        }
        if let Some(i) = best_idx {
            self.bins[i] += item;
        } else {
            self.bins.push(item);
        }
    }

    /// Implements the Worst-Fit strategy.
    fn _worst_fit(&mut self, item: f64) {
        let mut worst_idx: Option<usize> = None;
        let mut worst_space = -1.0;
        for (i, &load) in self.bins.iter().enumerate() {
            let space = BIN_CAPACITY - load;
            if space >= item && space > worst_space {
                worst_space = space;
                worst_idx = Some(i);
            }
        }
        if let Some(i) = worst_idx {
            self.bins[i] += item;
        } else {
            self.bins.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    #[test]
    fn test_next_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::NextFit);
        for &item in &[0.5, 0.5, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.25]);
    }

    #[test]
    fn test_random_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::RandomFit(rng()));
        for _ in 0..4 {
            mgr.add_item(0.5);
        }
        assert_eq!(mgr.bins().len(), 2);
        for &load in mgr.bins() {
            assert!(load <= BIN_CAPACITY);
        }
    }

    #[test]
    fn test_first_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::FirstFit);
        for &item in &[0.75, 0.5, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.5]);
    }

    #[test]
    fn test_best_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::BestFit);
        for &item in &[0.5, 0.25, 0.75, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.75]);
    }

    #[test]
    fn test_worst_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::WorstFit);
        for &item in &[0.5, 0.25, 0.75, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.75]);
    }
}