use rand::seq::IteratorRandom;
use rand::rngs::ThreadRng;

const BIN_CAPACITY: f64 = 1.0;

#[derive(Debug, Clone)]
pub enum PackingStrategy {
    NextFit,
    RandomFit(ThreadRng),
    FirstFit,
    BestFit,
    WorstFit,
}

#[derive(Debug)]
pub struct BinPackingManager {
    strategy: PackingStrategy,
    bins: Vec<f64>,
}

impl BinPackingManager {
    pub fn new(strategy: PackingStrategy) -> Self {
        BinPackingManager {
            strategy,
            bins: Vec::new(),
        }
    }

    pub fn bins(&self) -> &[f64] {
        &self.bins
    }

    pub fn add_item(&mut self, item: f64) {
        match self.strategy {
            PackingStrategy::NextFit => self._next_fit(item),
            PackingStrategy::RandomFit(_) => self._random_fit(item),
            PackingStrategy::FirstFit => self._first_fit(item),
            PackingStrategy::BestFit => self._best_fit(item),
            PackingStrategy::WorstFit => self._worst_fit(item),
        }
    }

    fn _next_fit(&mut self, item: f64) {
        if let Some(last) = self.bins.last_mut() {
            if *last + item <= BIN_CAPACITY {
                *last += item;
                return;
            }
        }
        // else start a new bin
        self.bins.push(item);
    }

    fn _random_fit(&mut self, item: f64) {
        // extract rng
        if let PackingStrategy::RandomFit(ref mut rng) = self.strategy {
            // collect all indices where item fits
            let candidates = self
                .bins
                .iter_mut()
                .enumerate()
                .filter(|(_, load)| **load + item <= BIN_CAPACITY);
            if let Some((_, load)) = candidates
                .choose(rng)
                .map(|(i, l)| (i, l))
            {
                *load += item;
            } else {
                self.bins.push(item);
            }
        }
    }

    fn _first_fit(&mut self, item: f64) {
        for load in &mut self.bins {
            if *load + item <= BIN_CAPACITY {
                *load += item;
                return;
            }
        }
        self.bins.push(item);
    }

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
        // 0.5 -> bin0=[0.5]
        // 0.5 -> fits in bin0 => [1.0]
        // 0.25 -> doesn't fit, new bin => [1.0, 0.25]
        for &item in &[0.5, 0.5, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.25]);
    }

    #[test]
    fn test_random_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::RandomFit(rng()));
        // Four 0.5's → each bin can hold exactly two → 2 bins total
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
        // 0.75 → bin0
        // 0.5  → new bin1 (0.75+0.5>1.0)
        // 0.25 → fits bin0 => [1.0, 0.5]
        for &item in &[0.75, 0.5, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.5]);
    }

    #[test]
    fn test_best_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::BestFit);
        // 0.5  → bin0=[0.5]
        // 0.25 → bin0=[0.75]
        // 0.75 → new bin1=[0.75]
        // 0.25 → best fit is bin0 (leftover=0) over bin1 ⇒ [1.0, 0.75]
        for &item in &[0.5, 0.25, 0.75, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.75]);
    }

    #[test]
    fn test_worst_fit() {
        let mut mgr = BinPackingManager::new(PackingStrategy::WorstFit);
        // 0.5  → bin0=[0.5]
        // 0.25 → worst fit is bin0 ⇒ [0.75]
        // 0.75 → doesn't fit in bin0, new bin1=[0.75]
        // 0.25 → worst fit is bin0 (space=0.25) ⇒ [1.0, 0.75]
        for &item in &[0.5, 0.25, 0.75, 0.25] {
            mgr.add_item(item);
        }
        assert_eq!(mgr.bins(), &[1.0, 0.75]);
    }
}