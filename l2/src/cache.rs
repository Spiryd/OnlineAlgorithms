use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::{Rng, seq::IndexedRandom};
use std::collections::{HashMap, VecDeque};

/// Enum representing different cache management strategies.
#[derive(Debug)]
pub enum CacheManagementStrategy {
    /// First-In-First-Out strategy: Evicts the oldest page when full.
    FIFO,
    /// Flush-When-Full strategy: Clears the entire cache when full and a miss occurs.
    FWF,
    /// Least-Recently-Used strategy: Tracks usage order to evict the least recently used page.
    LRU(VecDeque<usize>),
    /// Least-Frequently-Used strategy: Tracks access frequencies to evict the least frequently used page.
    LFU(HashMap<usize, usize>),
    /// Random strategy: Evicts a random page when full.
    RAND(ThreadRng),
    /// RANDOMIZED MARKUP ALGORITHM: Evicts a page based on a randomized algorithm.
    RMA(HashMap<usize, bool>, ThreadRng),
}

/// Struct representing a cache manager that handles page requests based on a given strategy.
#[derive(Debug)]
pub struct CacheManager {
    /// The cache management strategy being used.
    strategy: CacheManagementStrategy,
    /// The maximum capacity of the cache.
    capacity: usize,
    /// The current memory (pages) stored in the cache.
    memory: VecDeque<usize>,
}

impl CacheManager {
    /// Creates a new `CacheManager` with the specified capacity and strategy.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The maximum number of pages the cache can hold.
    /// * `strategy` - The cache management strategy to use.
    ///
    /// # Returns
    ///
    /// A new instance of `CacheManager`.
    pub fn new(capacity: usize, strategy: CacheManagementStrategy) -> Self {
        CacheManager {
            strategy,
            capacity,
            memory: VecDeque::with_capacity(capacity),
        }
    }

    /// Accesses a page in the cache and returns the cost of the access.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access (0 for a hit, 1 for a miss).
    pub fn access(&mut self, page_id: usize) -> usize {
        match &mut self.strategy {
            CacheManagementStrategy::FIFO => self._fifo_access(page_id),
            CacheManagementStrategy::FWF => self._fwf_access(page_id),
            CacheManagementStrategy::LRU(_) => self._lru_access(page_id),
            CacheManagementStrategy::LFU(_) => self._lfu_access(page_id),
            CacheManagementStrategy::RAND(_) => self._random_access(page_id),
            CacheManagementStrategy::RMA(_, _) => self._rma_access(page_id),
        }
    }

    /// Handles page access using the FIFO strategy.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access.
    fn _fifo_access(&mut self, page_id: usize) -> usize {
        if self.memory.contains(&page_id) {
            0 // Hit: no cost
        } else {
            if self.memory.len() == self.capacity {
                self.memory.pop_front(); // Evict oldest
            }
            self.memory.push_back(page_id); // Add new page
            1 // Miss: cost = 1
        }
    }

    /// Handles page access using the FWF strategy.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access.
    fn _fwf_access(&mut self, page_id: usize) -> usize {
        if self.memory.contains(&page_id) {
            0 // Hit
        } else {
            if self.memory.len() == self.capacity {
                self.memory.clear(); // Flush entire memory
            }
            self.memory.push_back(page_id); // Add the requested page
            1 // Miss
        }
    }

    /// Handles page access using the LRU strategy.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access.
    fn _lru_access(&mut self, page_id: usize) -> usize {
        if let CacheManagementStrategy::LRU(usage_order) = &mut self.strategy {
            if self.memory.contains(&page_id) {
                // Move to most recently used
                if let Some(pos) = usage_order.iter().position(|&x| x == page_id) {
                    usage_order.remove(pos);
                }
                usage_order.push_back(page_id);
                0
            } else {
                // Miss: possibly evict
                if self.memory.len() == self.capacity {
                    if let Some(lru) = usage_order.pop_front() {
                        if let Some(pos) = self.memory.iter().position(|&x| x == lru) {
                            self.memory.remove(pos);
                        }
                    }
                }
                self.memory.push_back(page_id);
                usage_order.push_back(page_id);
                1
            }
        } else {
            panic!("_lru_access called with non-LRU strategy");
        }
    }

    /// Handles page access using the LFU strategy.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access.
    fn _lfu_access(&mut self, page_id: usize) -> usize {
        if let CacheManagementStrategy::LFU(freq_map) = &mut self.strategy {
            if self.memory.contains(&page_id) {
                // Hit: increase frequency
                *freq_map.entry(page_id).or_insert(0) += 1;
                0
            } else {
                // Miss
                if self.memory.len() == self.capacity {
                    // Find LFU page
                    if let Some((lfu_page, _)) = self
                        .memory
                        .iter()
                        .min_by_key(|&&pid| freq_map.get(&pid).copied().unwrap_or(0))
                        .map(|&pid| (pid, freq_map.get(&pid).copied().unwrap_or(0)))
                    {
                        // Remove LFU page
                        if let Some(pos) = self.memory.iter().position(|&x| x == lfu_page) {
                            self.memory.remove(pos);
                        }
                        freq_map.remove(&lfu_page);
                    }
                }

                self.memory.push_back(page_id);
                freq_map.insert(page_id, 1);
                1
            }
        } else {
            panic!("_lfu_access called with non-LFU strategy");
        }
    }

    /// Handles page access using the Random (RAND) strategy.
    ///
    /// # Arguments
    ///
    /// * `page_id` - The ID of the page being accessed.
    ///
    /// # Returns
    ///
    /// The cost of the access:
    /// - `0` if the page is already in the cache (hit).
    /// - `1` if the page is not in the cache and needs to be added (miss).
    ///
    /// # Panics
    ///
    /// This function will panic if the cache management strategy is not `RAND`.
    fn _random_access(&mut self, page_id: usize) -> usize {
        if self.memory.contains(&page_id) {
            0 // Hit
        } else {
            if self.memory.len() == self.capacity {
                if let CacheManagementStrategy::RAND(rng) = &mut self.strategy {
                    // Evict a random page from the cache
                    let index = rng.random_range(0..self.memory.len());
                    self.memory.remove(index);
                } else {
                    panic!("_random_access called with non-RAND strategy");
                }
            }
            // Add the new page to the cache
            self.memory.push_back(page_id);
            1 // Miss
        }
    }

    /// RMA strategy: Randomized Markup Algorithm.
    /// On a hit, the page is marked.
    /// On a miss:
    ///   - If there's room, add the page and mark it.
    ///   - If full, evict an unmarked page chosen uniformly at random.
    ///   - If all pages are marked, clear marks and then evict one at random.
    fn _rma_access(&mut self, page_id: usize) -> usize {
        if let CacheManagementStrategy::RMA(mark_map, rng) = &mut self.strategy {
            if self.memory.contains(&page_id) {
                // Hit: mark the page.
                mark_map.insert(page_id, true);
                0
            } else {
                if self.memory.len() < self.capacity {
                    self.memory.push_back(page_id);
                    mark_map.insert(page_id, true);
                    1
                } else {
                    // Cache is full.
                    let unmarked: Vec<usize> = self
                        .memory
                        .iter()
                        .cloned()
                        .filter(|pid| !*mark_map.get(pid).unwrap_or(&false))
                        .collect();
                    let victim = if !unmarked.is_empty() {
                        unmarked.choose(rng).copied().unwrap()
                    } else {
                        // All pages are marked; clear marks.
                        for pid in self.memory.iter() {
                            mark_map.insert(*pid, false);
                        }
                        self.memory.iter().choose(rng).copied().unwrap()
                    };
                    if let Some(pos) = self.memory.iter().position(|&x| x == victim) {
                        self.memory.remove(pos);
                    }
                    mark_map.remove(&victim);
                    self.memory.push_back(page_id);
                    mark_map.insert(page_id, true);
                    1
                }
            }
        } else {
            panic!("_rma_access called with non-RMA strategy");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    /// Tests the FIFO strategy for correctness.
    #[test]
    fn test_fifo_strategy() {
        let requests = vec![1, 2, 3, 1, 4, 2, 5];
        let mut cache = CacheManager::new(3, CacheManagementStrategy::FIFO);
        let expected_costs = vec![1, 1, 1, 0, 1, 0, 1];

        for (i, &req) in requests.iter().enumerate() {
            let cost = cache.access(req);
            assert_eq!(cost, expected_costs[i], "Mismatch at request index {}", i);
        }
    }

    /// Tests the FWF strategy for correctness.
    #[test]
    fn test_fwf_strategy() {
        let requests = vec![1, 2, 3, 4, 1, 2, 5];
        let mut cache = CacheManager::new(3, CacheManagementStrategy::FWF);
        let expected_costs = vec![1, 1, 1, 1, 1, 1, 1];

        for (i, &req) in requests.iter().enumerate() {
            let cost = cache.access(req);
            assert_eq!(cost, expected_costs[i], "Mismatch at request index {}", i);
        }
    }

    /// Tests the LRU strategy for correctness.
    #[test]
    fn test_lru_strategy() {
        let requests = vec![1, 2, 3, 1, 4, 5];
        let mut cache = CacheManager {
            strategy: CacheManagementStrategy::LRU(VecDeque::new()),
            capacity: 3,
            memory: VecDeque::with_capacity(3),
        };

        let expected_costs = vec![1, 1, 1, 0, 1, 1];

        for (i, &req) in requests.iter().enumerate() {
            let cost = cache.access(req);
            assert_eq!(cost, expected_costs[i], "Mismatch at request index {}", i);
        }
    }

    /// Tests the LFU strategy for correctness.
    #[test]
    fn test_lfu_strategy() {
        let requests = vec![1, 2, 1, 3, 4, 1, 5];
        let mut cache = CacheManager {
            strategy: CacheManagementStrategy::LFU(HashMap::new()),
            capacity: 3,
            memory: VecDeque::with_capacity(3),
        };

        // Expected behavior:
        // - 1, 2, 1, 3  => miss, miss, hit (1 becomes freq 2), miss
        // - 4 replaces 2 (freq 1)
        // - 1 hit again (freq 3)
        // - 5 replaces 3 (freq 1)
        let expected_costs = vec![1, 1, 0, 1, 1, 0, 1];

        for (i, &req) in requests.iter().enumerate() {
            let cost = cache.access(req);
            assert_eq!(cost, expected_costs[i], "Mismatch at request index {}", i);
        }
    }

    /// Tests the Random strategy for correctness.
    #[test]
    fn test_random_strategy() {
        // Create a cache with RAND strategy using its own RNG.
        let mut cache = CacheManager::new(3, CacheManagementStrategy::RAND(rng()));
        // First access is a miss.
        assert_eq!(cache.access(1), 1);
        // Second access to the same page is a hit.
        assert_eq!(cache.access(1), 0);

        // Add two more pages to fill the cache.
        assert_eq!(cache.access(2), 1);
        assert_eq!(cache.access(3), 1);

        // Cache is full. Adding a new page (4) should cause a random eviction.
        assert_eq!(cache.access(4), 1);
        // Check that page 4 is now in the cache.
        assert_eq!(cache.access(4), 0);
    }

    #[test]
    fn test_rma_strategy() {
        let mut cache = CacheManager::new(3, CacheManagementStrategy::RMA(HashMap::new(), rng()));
        // First access: miss, add and mark.
        assert_eq!(cache.access(1), 1);
        // Second access: hit, already marked.
        assert_eq!(cache.access(1), 0);
        // Fill remaining cache.
        assert_eq!(cache.access(2), 1);
        assert_eq!(cache.access(3), 1);
        // Cache is full; accessing a new page (4) will trigger RMA eviction.
        assert_eq!(cache.access(4), 1);
        // Page 4 should now be in the cache.
        assert_eq!(cache.access(4), 0);
    }
}
